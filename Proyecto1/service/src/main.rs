use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use std::sync::mpsc;
use chrono::Utc;
use ctrlc;
use std::sync::{Arc, Mutex};
use std::process::{Command, Child, Stdio};
#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "memorystats")]
    memory_info: MemoryInfo,
    #[serde(rename = "processes")]
    processes: Vec<LogProcess>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MemoryInfo {
    #[serde(rename = "total_ram")]
    total_ram: u64,
    #[serde(rename = "free_ram")]
    free_ram: u64,
    #[serde(rename = "used_ram")]
    used_ram: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct LogProcess {
    #[serde(rename = "pid")]
    pid: u32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "cmdline")]
    cmd_line: String,
    #[serde(rename = "vsz")]
    vsz: u64,
    #[serde(rename = "rss")]
    rss: u64,
    #[serde(rename = "memory_usage")]
    memory_usage: f64,
    #[serde(rename = "cpu_usage")]
    cpu_usage: f64,
}

impl LogProcess {
    fn get_container_id(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Levantar el contenedor de Python y obtener su ID
    let python_container_id = match run_docker_compose() {
        Ok(_) => match get_python_container_id() {
            Ok(container_id) => {
                println!("Contenedor levantado con ID: {}", container_id);
                container_id  
            },
            Err(e) => {
                eprintln!("Error al obtener el ID del contenedor: {}", e);
                return Err(Box::from("No se pudo obtener el ID del contenedor de Python."));
            }
        },
        Err(e) => {
            eprintln!("Error al levantar el contenedor: {}", e);
            return Err(Box::from("No se pudo levantar el contenedor de Python."));
        }
    };
    //Correr el Cronjob
    let script_path = "/home/kev/Documentos/SO1_2S2024_201902278/Proyecto1/service/scripts/cronjob.sh"; 

    // Crear el proceso hijo
    let child_process = Arc::new(Mutex::new(Some(start_container_creation(script_path))));
    let child_process_clone = Arc::clone(&child_process);
    // Canal para manejar señales de interrupción (Ctrl+C)    
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    }).expect("Error setting Ctrl-C handler");

    let client = Client::new();
    println!("Servicio en Rust ejecutándose...");

    // Loop principal del servicio que se ejecuta hasta recibir una interrupción
    loop {
        match rx.try_recv() {
            Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                println!("Interrupción recibida. Terminando el servicio...");
            // Mata la creación de los scripts
            if let Some(mut child) = child_process_clone.lock().unwrap().take() {
                let _ = child.kill();
                println!("El proceso de creación del contenedor se detuvo.");
            }                  
                // Realizar la solicitud POST para generar las gráficas
                let response = client
                    .post("http://localhost:8000/generate_graphs")  
                    .send()
                    .await;

                match response {
                    Ok(res) => println!("Solicitud para generar gráficas realizada con éxito. Respuesta: {:?}", res),
                    Err(err) => eprintln!("Error enviando solicitud para generar gráficas: {}", err),
                }

                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Leer el archivo /proc/sysinfo_<carnet>
                let json_str = read_proc_file("/proc/sysinfo_201902278").unwrap();
                
                match serde_json::from_str::<SystemInfo>(&json_str) {
                    Ok(system_info) => {
                        // Separar los procesos y obtener la lista de contenedores a eliminar
                        let (high_consumption, low_consumption) = analyze(&system_info, &python_container_id).await;
                        let containers_to_kill = process_containers(high_consumption, low_consumption, &python_container_id).await;
                        
                        // Verifica si la lista de containers_to_kill está vacía o no
                        if containers_to_kill.is_empty() {
                            println!("No hay contenedores para eliminar.");
                        } else {
                            println!("Se eliminarán los siguientes contenedores: {:?}", containers_to_kill);
                        }

                        let payload = serde_json::json!({
                            "timestamp": Utc::now().to_rfc3339(),
                            "memorystats": system_info.memory_info,  // Usamos la referencia prestada
                            "processes": containers_to_kill  // Enviamos la lista de contenedores a eliminar
                        });
                
                        // Imprimir el JSON que se enviará
                        println!("JSON enviado: {}", payload);
                
                        // Enviar la información al servidor Python
                        if let Err(e) = send_logs_to_python_server(&client, payload).await {
                            eprintln!("Error enviando los logs: {}", e);
                        }
                
                        // Eliminar los contenedores después de enviarlos al servidor de logs
                        delete_containers(containers_to_kill, &python_container_id).await;
                    }
                    Err(e) => eprintln!("Error deserializando JSON: {}", e),
                }
                
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        }
    }

    Ok(())
}


// Función para leer el archivo /proc
fn read_proc_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Función para enviar los logs al servidor Python
async fn send_logs_to_python_server(client: &Client, payload: serde_json::Value) -> Result<(), reqwest::Error> {
    let url = "http://localhost:8000/logs";
    
    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Logs enviados exitosamente");
    } else {
        eprintln!("Error al enviar los logs: {}", response.status());
    }

    Ok(())
}
fn run_docker_compose() -> io::Result<String> {
    // Ejecuta el comando 'docker-compose up -d' para levantar el contenedor en segundo plano
    let output = Command::new("docker-compose")
        .arg("up")
        .arg("-d")  // Levanta el contenedor en segundo plano
        .current_dir("./python_server")  
        .output()?;

    if output.status.success() {
        println!("Contenedor de Python levantado correctamente.");
    } else {
        eprintln!("Error al levantar el contenedor: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Devuelve que funcionó o no
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
fn get_python_container_id() -> io::Result<String> {
    // Ejecuta 'docker ps' y filtra el contenedor basado en su nombre
    let output = Command::new("docker")
        .arg("ps")
        .arg("--filter")
        .arg("name=python_service")  // Nombre del contenedor
        .arg("--format")
        .arg("{{.ID}}")  // Obtén solo el ID del contenedor truncado
        .output()?;

    if output.status.success() {
        let container_id = String::from_utf8_lossy(&output.stdout).to_string().trim().to_string();
        println!("ID del contenedor de Python: {}", container_id);
        return Ok(container_id);
    } else {
        eprintln!("Error al obtener el ID del contenedor: {}", String::from_utf8_lossy(&output.stderr));
        return Err(io::Error::new(io::ErrorKind::Other, "No se pudo obtener el ID del contenedor."));
    }
}

// Función de análisis que separa los procesos en alto y bajo consumo y devuelve ambas listas
async fn analyze(system_info: &SystemInfo, python_container_id: &str) -> (Vec<LogProcess>, Vec<LogProcess>) {
    let mut high_consumption: Vec<LogProcess> = Vec::new();
    let mut low_consumption: Vec<LogProcess> = Vec::new();

    // Filtrar procesos que no sean el contenedor de Python
    for process in &system_info.processes {
        if process.get_container_id().contains(python_container_id) {
            println!("Excluyendo contenedor de logs (Python) con ID: {}", python_container_id);
            continue;  // Excluir el contenedor Python
        }

        // Separar en alto y bajo consumo según el uso de CPU
        if process.cpu_usage > 0.08 {
            high_consumption.push(process.clone());
        } else {
            low_consumption.push(process.clone());
        }
    }

    // Devolver las listas de alto y bajo consumo
    (high_consumption, low_consumption)
}


// Función para procesar los contenedores de alto y bajo consumo y devolver los contenedores a eliminar
async fn process_containers(mut high_consumption: Vec<LogProcess>, mut low_consumption: Vec<LogProcess>, python_container_id: &str) -> Vec<LogProcess> {
    let mut containers_to_kill: Vec<LogProcess> = Vec::new(); // Lista de contenedores a eliminar

    // Ordenar los procesos de alto consumo de mayor a menor uso de CPU
    high_consumption.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    // Ordenar los procesos de bajo consumo de menor a mayor uso de CPU
    low_consumption.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap());

    // Asegurar que haya al menos 2 contenedores de alto consumo
    while high_consumption.len() < 2 {
        // Si hay menos de 2, añadir "contenedores ficticios" con consumo bajo/nulo para llenar la lista
        high_consumption.push(LogProcess {
            pid: 0,
            name: "placeholder_high".to_string(),
            cmd_line: "".to_string(),
            vsz: 0,
            rss: 0,
            memory_usage: 0.0,
            cpu_usage: 0.0,
        });
    }

    // Asegurar que haya al menos 3 contenedores de bajo consumo
    while low_consumption.len() < 3 {
        // Si hay menos de 3, añadir "contenedores ficticios" con consumo bajo/nulo para llenar la lista
        low_consumption.push(LogProcess {
            pid: 0,
            name: "placeholder_low".to_string(),
            cmd_line: "".to_string(),
            vsz: 0,
            rss: 0,
            memory_usage: 0.0,
            cpu_usage: 0.0,
        });
    }

    // Mantener solo los primeros 2 en la lista de alto consumo
    if high_consumption.len() > 2 {
        let mut to_kill = high_consumption.split_off(2);
        // Evitar eliminar el contenedor Python
        to_kill.retain(|container| container.get_container_id() != python_container_id);
        containers_to_kill.extend(to_kill); // Los que quedan fuera van a la lista para eliminar
    }

    // Mantener solo los primeros 3 en la lista de bajo consumo
    if low_consumption.len() > 3 {
        let mut to_kill = low_consumption.split_off(3);
        // Evitar eliminar el contenedor Python
        to_kill.retain(|container| container.get_container_id() != python_container_id);
        containers_to_kill.extend(to_kill); // Los que quedan fuera van a la lista para eliminar
    }

    // Eliminar los contenedores ficticios de la lista de eliminación
    containers_to_kill.retain(|container| container.pid != 0);

    // Devuelve la lista de contenedores a eliminar, asegurando que no se incluya el contenedor de Python
    containers_to_kill
}

// Función para eliminar los contenedores de la lista
async fn delete_containers(containers_to_kill: Vec<LogProcess>, python_container_id: &str) {
    for process in containers_to_kill {
        let container_id = process.get_container_id();

        // Verificar si el contenedor es el contenedor Python, y si es así, evitar eliminarlo
        if container_id == python_container_id {
            println!("El contenedor de logs (Python) con ID: {} no será eliminado.", container_id);
            continue;
        }

        println!(
            "Eliminando contenedor con ID: {}, PID: {}, Nombre: {}",
            container_id, process.pid, process.name
        );

        // Llamar a la función de eliminación
        let _output = kill_container(container_id);
    }
}


// Función para eliminar un contenedor
fn kill_container(container_id: &str) -> io::Result<()> {
    let output = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(container_id)
        .output()?;

    if output.status.success() {
        println!("Contenedor {} eliminado exitosamente.", container_id);
    } else {
        eprintln!("Error al eliminar el contenedor {}: {}", container_id, String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
fn start_container_creation(script_path: &str) -> Child {
    let log_file = File::create("./contenedores.log")
        .expect("Error en crear el archivo de log"); 
    let child = Command::new(script_path)
        .stdout(Stdio::from(
            log_file
                .try_clone()
                .expect("Failed to clone log file for stdout") 
        ))
        .stderr(Stdio::from(log_file)) 
        .spawn() 
        .expect("Failed to execute container creation script"); 
    child
}