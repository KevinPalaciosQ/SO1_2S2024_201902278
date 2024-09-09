use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use ctrlc;
use reqwest::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "MemoryStats")]
    system_info: MemoryInfo,
    #[serde(rename = "Processes")]
    processes: Vec<Process>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "Vsz")]  // Virtual Set Size (en bytes)
    vsz: String,
    #[serde(rename = "Rss")]  // Resident Set Size (en bytes)
    rss: String,   
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MemoryInfo {
    #[serde(rename = "Total_RAM")]
    total_ram: u64,
    #[serde(rename = "Free_RAM")]
    free_ram: u64,
    #[serde(rename = "Used_RAM")]
    used_ram: u64
}

#[derive(Debug, Serialize, Clone)]
struct MEMO {
    total_ram: u64,
    free_ram: u64,
    used_ram: u64,
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    memory_usage: f64,
    cpu_usage: f64,
    vsz: String,  
    rss: String,    
}

impl Process {
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

impl Eq for Process {}

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.memory_usage.partial_cmp(&other.memory_usage).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn kill_container(id: &str) -> std::process::Output {
    let output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Finalizando el contenedor con id: {}", id);

    output
}



fn analyzer(system_info: SystemInfo) {
    println!("Total RAM: {}, Free RAM: {}, Used RAM: {}", system_info.system_info.total_ram, system_info.system_info.free_ram, system_info.system_info.used_ram);
    
    tokio::spawn(async {
        if let Err(e) = post_memory_info(system_info.system_info).await {
            eprintln!("Error al enviar logs: {}", e);
        }
    });

    let mut log_proc_list: Vec<LogProcess> = Vec::new();

    let mut processes_list: Vec<Process> = system_info.processes;
    let mut highest_list: Vec<Process> = Vec::new();
    let mut lowest_list: Vec<Process> = Vec::new();

    processes_list.sort();

    for process in processes_list.iter() {
        let entrar = process.clone();
        if  process.cpu_usage > 0.08 {
            highest_list.push(entrar);
        } else {
            lowest_list.push(entrar);
        }
    }

    println!(" Contenedores de Bajo consumo:");
    for process in &lowest_list {
        println!(
            "PID: {}, Name: {}, Container ID: {}, Vsz: {}, Rss: {}, Memory Usage: {}, CPU Usage: {}",
            process.pid, 
            process.name, 
            process.get_container_id(), 
            process.vsz,    // Imprimir el valor de Vsz
            process.rss,    // Imprimir el valor de Rss
            process.memory_usage, 
            process.cpu_usage
        );
    }
    
    println!("------------------------------");
    
    println!("Contendores de Alto consumo:");
    for process in &highest_list {
        println!(
            "PID: {}, Name: {}, Container ID: {}, Vsz: {}, Rss: {}, Memory Usage: {}, CPU Usage: {}",
            process.pid, 
            process.name, 
            process.get_container_id(), 
            process.vsz,    // Imprimir el valor de Vsz
            process.rss,    // Imprimir el valor de Rss
            process.memory_usage, 
            process.cpu_usage
        );
    }
    

    println!("------------------------------");

    if lowest_list.len() > 3 {
        for process in lowest_list.iter().skip(3) {
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                vsz: process.vsz.clone(),  // Capturar el valor de Vsz
                rss: process.rss.clone(),  // Capturar el valor de Rss               
            };

            log_proc_list.push(log_process.clone());
            let _output = kill_container(&process.get_container_id());
        }
    }
    if highest_list.len() > 2 {
        for process in highest_list.iter().take(highest_list.len() - 2) {
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                vsz: process.vsz.clone(),  // Capturar el valor de Vsz
                rss: process.rss.clone(),  // Capturar el valor de Rss                
            };

            log_proc_list.push(log_process.clone());
            let _output = kill_container(&process.get_container_id());
        }
    }

    println!("Contenedores matados");
    for process in log_proc_list.iter() {
        println!(
            "PID: {}, Name: {}, Container ID: {}, Vsz: {}, Rss: {}, Memory Usage: {}, CPU Usage: {}",
            process.pid,
            process.name,
            process.container_id,
            process.vsz,          
            process.rss,          
            process.memory_usage,
            process.cpu_usage
        );
    }

    println!("------------------------------");

    // Enviar los logs al servidor FastAPI
    tokio::spawn(async {
        if let Err(e) = send_logs_to_server(log_proc_list).await {
            eprintln!("Error al enviar logs: {}", e);
        }
    });
}

fn read_proc_file(file_name: &str) -> io::Result<String> {
    let path = Path::new("/proc").join(file_name);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    let system_info: SystemInfo = serde_json::from_str(json_str)?;
    Ok(system_info)
}



#[tokio::main]
async fn main() -> Result<(), Error>  {

    loop {
        
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(Mutex::new(tx));

        // Configurar el manejador de señales Ctrl+C
        let tx_clone = tx.clone();
        ctrlc::set_handler(move || {
            let _ = tx_clone.lock().unwrap().send(());
        }).expect("Error al configurar el manejador de señales");

        println!("Presiona Ctrl+C para terminar el programa.");

        // Ejecutar una tarea de fondo que puede ser interrumpida
        loop {
            match rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Interrupción recibida. El programa se está terminando.");
                    let url = "http://localhost:8000/graph"; 
                    
                    let response = reqwest::get(url).await?;

                    // Verifica el código de estado HTTP
                    if response.status().is_success() {
                        // Obtiene el contenido de la respuesta como texto
                        let body = response.text().await?;
                        println!("Respuesta: {}", body);
                    } else {
                        println!("Error en la solicitud: {}", response.status());
                    }

                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    let system_info: Result<SystemInfo, _>;
                    let json_str = read_proc_file("sysinfo_201902278").unwrap();
                    system_info = parse_proc_to_struct(&json_str);


            
                    match system_info {
                        Ok(info) => {
                            analyzer(info); // Esperar a que se complete la tarea
                        }
                        Err(e) => println!("Failed to parse JSON: {}", e),
                    }
            
            
                    std::thread::sleep(std::time::Duration::from_secs(10));
                    
                }
            }
        }


    
    }
}