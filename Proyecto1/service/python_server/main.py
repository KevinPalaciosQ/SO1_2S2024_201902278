import matplotlib.pyplot as plt
import logging
from fastapi import FastAPI, HTTPException
import os
import json
from models.models import Data

# Configurar logging para FastAPI
logging.basicConfig(level=logging.DEBUG)

app = FastAPI()

@app.post("/logs")
def get_logs(logs_proc: Data):
    logging.debug(f"Datos recibidos: {logs_proc}")
    
    if not os.path.exists('logs'):
        os.makedirs('logs')
    
    logs_file = 'logs/logs.json'
    
    try:
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    except FileNotFoundError:
        logging.warning("Archivo de logs no encontrado. Creando uno nuevo.")
        existing_logs = []
    except json.JSONDecodeError:
        logging.error("Error decodificando JSON desde el archivo de logs")
        existing_logs = []
    
    try:
        logging.debug(f"Procesando los datos recibidos: {logs_proc.dict()}")
        existing_logs.append(logs_proc.dict())

        # Guardar los logs actualizados en el archivo
        with open(logs_file, 'w') as file:
            json.dump(existing_logs, file, indent=4)
        
        logging.info("Logs guardados exitosamente.")
        return {"received": True}
    
    except Exception as e:
        logging.error(f"Error procesando los datos: {e}")
        raise HTTPException(status_code=422, detail=f"Error procesando los datos: {e}")




@app.post("/generate_graphs")
def generate_graphs():
    logs_file = 'logs/logs.json'

    try:
        with open(logs_file, 'r') as file:
            logs = json.load(file)
    except FileNotFoundError:
        return {"error": "No logs found"}
    
    if logs:
        timestamps = [log['timestamp'] for log in logs]

        used_ram = [log['memorystats']['used_ram'] for log in logs]

        cpu_usage = []
        for log in logs:
            if 'processes' in log and log['processes']:
                # Sumamos el uso de CPU de todos los procesos para ese log
                total_cpu = sum([process['cpu_usage'] for process in log['processes']])
                cpu_usage.append(total_cpu)
            else:
                cpu_usage.append(0)  # Si no hay procesos el uso de CPU es 0

        if len(timestamps) > 1:  # Verificar que hayan suficientes datos para graficar
            # Graficar el uso de memoria (RAM)
            plt.figure(figsize=(10, 6))
            plt.plot(timestamps, used_ram, color='green', marker='o', linestyle='-')
            plt.xlabel('Time')
            plt.ylabel('Used RAM')
            plt.title('Memory Usage Over Time')
            plt.xticks(rotation=45, ha='right')
            plt.tight_layout()
            plt.savefig('logs/memory_usage_over_time.png')
            plt.close()

            # Graficar el uso de CPU
            plt.figure(figsize=(10, 6))
            plt.plot(timestamps, cpu_usage, color='blue', marker='o', linestyle='-')
            plt.xlabel('Time')
            plt.ylabel('CPU Usage (%)')
            plt.title('CPU Usage Over Time')
            plt.xticks(rotation=45, ha='right')
            plt.tight_layout()
            plt.savefig('logs/cpu_usage_over_time.png')
            plt.close()
        else:
            return {"error": "Not enough data points to generate graph"}
    
    return {"graphs": "generated"}