selected_images=()
for ((i=0; i<10; i++)); do
    selected_images+=("${images[RANDOM % ${#images[@]}]}")
done

# Crear 10 contenedores
for image in "${selected_images[@]}"; do
    docker run -d "$image"
done