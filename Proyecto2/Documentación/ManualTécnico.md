# 🎓 Universidad de San Carlos de Guatemala

# 💻 Ingeniería en Ciencias y Sistemas

# 👨‍🏫 Ing. Jesus Alberto Guzman Polanco

# 👨‍🏫 Aux. Alvaro Garcia

# 🏫 Sección A

# 📂 Proyecto 2

| 🎓Nombre                         | 📛Carné   |
| -------------------------------- | --------- |
| Kevin Estuardo Palacios Quiñonez | 201902278 |

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
El objetivo del proyecto Olimpiadas USAC es implementar una arquitectura en la nube, utilizando Google Kubernetes Engine (GKE) en Google Cloud Platform (GCP), para gestionar y monitorear las competencias de las Olimpiadas de la Universidad de San Carlos de Guatemala. El sistema debe ser capaz de manejar altos volúmenes de tráfico y permitir el monitoreo en tiempo real del desempeño de las facultades participantes en disciplinas como Natación, Boxeo y Atletismo, mostrando los resultados en Grafana.
# 👷🏻 Arquitectura del Proyecto
![arquitectura](/Proyecto2/images/Arquitectura.png)
# 📍 Tecnologías
#### Estas son las tecnologías y herramientas utilizadas en el proyecto:
- **locust:** locust 3.12.3
- **golang:** golang 1.23.2
- **rust:** rustc 1.80.1
- **gRPC**: Framework de comunicación de alta eficiencia 
utilizado para la comunicación entre servicios. **v1.67.1**.
- **Grafana**: Herramienta de visualización y análisis de datos. **Versión: 10.1.0**.
- **Redis**: Almacenamiento en memoria utilizado como base de datos y caché. **Versión: 7.0.12**.
- **Prometheus**: Sistema de monitoreo y alerta para aplicaciones. **Versión: 2.42.0**.

# 🛠️ Herramientas
- Visual Studio Code 1.90.1
- Consola Google Cloud Platform
# 🛠️ Componentes Utilizados
El proyecto utiliza una combinación de tecnologías modernas y robustas para crear un sistema de monitoreo eficiente y de alto rendimiento. Se despliega en Google Cloud Platform (GCP) utilizando Google Kubernetes Engine (GKE) para la orquestación de contenedores. El lenguaje de programación Golang se emplea para desarrollar servicios concurrentes, maximizando la eficiencia del sistema.

# 🔧 Comandos Utilizados
# Proyecto Olimpiadas USAC

## Configuración de Google Cloud SDK

```bash
# Proyecto Olimpiadas USAC

## Configuración de Google Cloud SDK
# Configura el proyecto en GCP
gcloud config set project olympiadas-usac-project

# Configura la zona
gcloud config set compute/zone us-central1-a
```
## Creación del clúster de Kubernetes
```bash
# Crea un clúster de Kubernetes
gcloud container clusters create olympiadas-cluster --num-nodes=3
```

## Conectar kubectl al clúster
```bash
# Obtener las credenciales del clúster
gcloud container clusters get-credentials olympiadas-cluster
```
##  Despliegue de Kafka usando Strimzi
```bash
# Instalar el Custom Resource Definition (CRD) de Strimzi
kubectl apply -f https://strimzi.io/install/latest/strimzi-cluster-operator.yaml

# Crear un Kafka cluster
kubectl apply -f kafka-cluster.yaml
```
## kafka-cluster.yaml
```yml
apiVersion: kafka.strimzi.io/v1beta2
kind: Kafka
metadata:
  name: olympiadas-kafka
spec:
  kafka:
    replicas: 3
    listeners:
      plain: {}
      tls: {}
    storage:
      type: persistent-claim
      size: 10Gi
  zookeeper:
    replicas: 3
    storage:
      type: persistent-claim
      size: 10Gi
```
## Despliegue de Redis

```bash
# Desplegar Redis
kubectl create deployment redis --image=redis
kubectl expose deployment redis --port=6379 --type=ClusterIP
```
## Despliegue de Grafana

```bash
# Añadir el repositorio de Grafana
helm repo add grafana https://grafana.github.io/helm-charts

# Instalar Grafana
helm install grafana grafana/grafana
```
## Despliegue de Prometheus

```bash
# Añadir el repositorio de Prometheus
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts

# Instalar Prometheus
helm install prometheus prometheus-community/prometheus
```
## Despliegue de servicios en Go y Rust

```bash
# Desplegar servicios de facultades
kubectl apply -f faculty-service.yaml

# Desplegar servicios de disciplinas
kubectl apply -f discipline-service.yaml
```
## faculty-service.yaml
```bash
apiVersion: apps/v1
kind: Deployment
metadata:
  name: faculty-service
spec:
  replicas: 2
  selector:
    matchLabels:
      app: faculty-service
  template:
    metadata:
      labels:
        app: faculty-service
    spec:
      containers:
      - name: faculty-service
        image: your-docker-repo/faculty-service:latest
        ports:
        - containerPort: 8080
---
apiVersion: v1
kind: Service
metadata:
  name: faculty-service
spec:
  type: ClusterIP
  ports:
  - port: 8080
    targetPort: 8080
  selector:
    app: faculty-service
```
## discipline-service.yaml
```bash
apiVersion: apps/v1
kind: Deployment
metadata:
  name: discipline-service
spec:
  replicas: 2
  selector:
    matchLabels:
      app: discipline-service
  template:
    metadata:
      labels:
        app: discipline-service
    spec:
      containers:
      - name: discipline-service
        image: your-docker-repo/discipline-service:latest
        ports:
        - containerPort: 8081
---
apiVersion: v1
kind: Service
metadata:
  name: discipline-service
spec:
  type: ClusterIP
  ports:
  - port: 8081
    targetPort: 8081
  selector:
    app: discipline-service
```
## Configuración del Horizontal Pod Autoscaler (HPA)

```bash
# Crear un HPA para un servicio específico
kubectl autoscale deployment faculty-service --cpu-percent=50 --min=1 --max=10
```
### Verificar el estado de los pods
kubectl get pods

# Verificar el estado de los servicios
```bash
# Verificar el estado de los pods
kubectl get pods

```

# Flujo del Proyecto
1. Locust genera tráfico.
2. Ingress redirige las peticiones a los servicios de facultades.
3. Servicios de facultades envían solicitudes a los servicios de disciplinas.
4. Servicios de disciplinas determinan ganadores/perdedores y publican resultados en Kafka.
5. Consumidores procesan resultados y almacenan en Redis.
6. Grafana visualiza los resultados en tiempo real.
7. Prometheus monitorea y recopila métricas del clúster.
