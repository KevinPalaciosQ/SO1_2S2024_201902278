# 🎓 Universidad de San Carlos de Guatemala
# 💻 Ingeniería en Ciencias y Sistemas
# 👨‍🏫 Ing. Jesus Alberto Guzman Polanco
# 👨‍🏫 Aux. Alvaro Garcia
# 🏫 Sección A
# 📂 Proyecto 1
|🎓Nombre                           |📛Carné       |
|-----------------------------------|--------------|
| Kevin Estuardo Palacios Quiñonez  | 201902278    |
# Manual Técnico
## 📚 Contenido
1. [🎯 Objetivo](#-objetivo-del-proyecto)
2. [🚀 Arquitectura del Proyecto](#-arquitectura-del-proyecto)
3. [📃 Requerimientos](#)
    - [📍Tecnologias](#-tecnologías)
    - [📍Herramientas](#️-herramientas)
    - [🛠️Componentes Utilizados](#️-componentes-utilizados)
4. [🖥 Flujo de la aplicación](#)
    - [Comandos Utilizados](#-comandos-utilizados) 
    - [Flujo de proyecto](#-flujo-de-proyecto)
    - [Aplicacion web](#-aplicacion-web)

# 🎯 Objetivo del Proyecto
El objetivo del proyecto es crear un sistema que gestione contenedores con Docker, automatizando su creación y monitoreo, y usando Rust para analizar su uso de recursos como memoria y CPU. También se deben generar logs y gráficas que muestren el rendimiento de los contenedores​
# 👷🏻 Arquitectura del Proyecto
![arquitectura](/Proyecto1/images/arquitectura.jpeg)
# 📍 Tecnologías
#### Estas son las tecnologías y herramientas utilizadas en el proyecto:
- **Docker:** 24.0.7
- **Docker Compose:** 24.0.7
- **rust:** rustc 1.80.1
- **python:** 3.12.3
- **flask:** 3.12.3
- **Linux Mint:** Cinnamon 6.2.9 

# 🛠️ Herramientas
- **Visual Studio Code:** 1.90.1
- **Navegador Web**
# 🛠️ Componentes Utilizados
El proyecto usa una combinación de tecnologías modernas y robustar para crear un sistema de monitoreo eficiente y de alto rendimiento, utilizando docker. Incluye el uso de módulos de kernel para obtener datos del sistema, contenedores de gestión y despliegue, programación asíncrona para eficiencia.
### 🛡️ Módulos del Kernel de Linux:

- **Módulo de RAM y CPU**:
  - Archivo en `/proc/sysinfo_201902278`
  - Librería: `<linux/module.h>`
### 🔙 Backend:

- **API**:
  - Lenguaje: Python-flask 🐍

### 🐳 Contenedores:

- **Plataforma de Contenedores**: Docker
# 🔧 Comandos Utilizados

Estos comandos proporcionan una guía básica para la instanciación y configuración de los componentes necesarios para la plataforma de monitoreo de procesos en un entorno Linux.

##### **Módulos del Kernel de Linux:**

```bash
make
sudo insmod sysinfo_201902278.ko
lsmod | grep sysinfo_201902278
```
### 🐳Contenedores 
**Instalar Docker:**
```bash
sudo apt update
sudo apt install docker.io
sudo systemctl start docker
sudo systemctl enable docker
```

### Creación del Backend con Python  🐍
**Instalar Python:**
```bash
mkdir servidor && cd servidor && python3 -m venv venv && source venv/bin/activate && pip install Flask && echo -e 'FROM python:3.9-slim\nWORKDIR /app\nCOPY . /app\nRUN pip install Flask\nCMD ["flask", "run", "--host=0.0.0.0"]' > Dockerfile
```
### Creación del Servicio con Rust 
**Instalar Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source $HOME/.cargo/env && mkdir service && cd service && cargo init
```
# 🖥 Flujo de proyecto
# 1. Realizar la instalaciónd el modulo de Kernel 
# 2. Levantar Rust, ya que este ejecuta el cronjob y levanta con docker compose la imagen de pyhon.

