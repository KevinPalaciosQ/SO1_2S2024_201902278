#!/bin/bash

# Lista de imágenes
images=(
    "low.py"
    "high.py"
)

# Crear una lista de 10 imágenes seleccionadas aleatoriamente
selected_images=()
for ((i=0; i<10; i++)); do
    selected_images+=("${images[RANDOM % ${#images[@]}]}")
done

# Crear 10 contenedores
for image in "${selected_images[@]}"; do
    docker run -d "$image"
done
