use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use olimpics::olimpics_service_server::{OlimpicsService, OlimpicsServiceServer};
use olimpics::{Student, Result};
