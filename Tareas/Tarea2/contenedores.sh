#!/bin/bash

# Caracteres permitidos en los nombres aleatorios
chars=abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789

# Cantidad de contenedores a crear
num_containers=10

# Longitud del nombre aleatorio
name_length=8

# Generar y crear contenedores
for i in $(seq 1 $num_containers); do
    # Generar un nombre aleatorio
    name=$(cat /dev/urandom | tr -dc "$chars" | head -c $name_length)

    # Crear el contenedor usando la imagen Alpine con sudo
    sudo docker run -d --name "$name" alpine

    # Mostrar el nombre del contenedor creado
    echo "Contenedor $i creado con nombre: $name"
done
