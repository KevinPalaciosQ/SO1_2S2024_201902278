# 🎓 Universidad de San Carlos de Guatemala
# 💻 Ingeniería en Ciencias y Sistemas
# 👨‍🏫 Ing. Jesus Alberto Guzman Polanco
# 👨‍🏫 Aux. Alvaro Garcia
# 🏫 Sección A
# 📂 Proyecto 1
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

