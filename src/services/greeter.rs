use sea_orm::DatabaseConnection;
use template::greeter_server::{Greeter, GreeterServer};
use template::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod template {
    tonic::include_proto!("template");
}

#[derive(Default)]
pub struct GreeterService {
    #[allow(dead_code)]
    conn: DatabaseConnection,
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = template::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

impl GreeterService {
    pub fn create_server(conn: DatabaseConnection) -> GreeterServer<GreeterService> {
        let server = GreeterService { conn };
        GreeterServer::new(server)
    }
}
