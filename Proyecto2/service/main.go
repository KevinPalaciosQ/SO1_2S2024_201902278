package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net"
	"net/http"
	"studentpb"

	"google.golang.org/grpc"
)

// Estructura para manejar la información del estudiante en HTTP
type Student struct {
	Name       string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

// Implementación del servidor gRPC
type studentServiceServer struct {
	studentpb.UnimplementedStudentServiceServer
}

// Implementación del método SendStudentData para el servicio gRPC
func (s *studentServiceServer) SendStudentData(ctx context.Context, req *studentpb.Student) (*studentpb.Response, error) {
	response := fmt.Sprintf("Received student: %+v\n", req)
	return &studentpb.Response{Message: response}, nil
}

func handler(w http.ResponseWriter, r *http.Request) {
	var student Student
	err := json.NewDecoder(r.Body).Decode(&student)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Configura la conexión gRPC
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect to gRPC server: %v", err)
	}
	defer conn.Close()

	client := studentpb.NewStudentServiceClient(conn)
	grpcReq := &studentpb.Student{
		Name:       student.Name,
		Age:        int32(student.Age),
		Faculty:    student.Faculty,
		Discipline: int32(student.Discipline),
	}

	// Llama al método gRPC
	resp, err := client.SendStudentData(context.Background(), grpcReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	fmt.Fprintf(w, "Response from gRPC server: %s\n", resp.Message)
}

func main() {
	// Servidor HTTP en una goroutine
	go func() {
		http.HandleFunc("/getStudent", handler)
		fmt.Println("HTTP server running on port 8080...")
		log.Fatal(http.ListenAndServe(":8080", nil))
	}()

	// Inicia el servidor gRPC
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	studentpb.RegisterStudentServiceServer(grpcServer, &studentServiceServer{})
	fmt.Println("gRPC server running on port 50051...")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
