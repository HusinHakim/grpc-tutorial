pub mod grpc_tutorial {
    tonic::include_proto!("grpc_tutorial");
}

use tonic::{transport::Server, Request, Response, Status};
use grpc_tutorial::payment_service_server::{PaymentService, PaymentServiceServer};
use grpc_tutorial::transaction_service_server::{TransactionService, TransactionServiceServer};
use grpc_tutorial::chat_service_server::{ChatService, ChatServiceServer};
use grpc_tutorial::{PaymentRequest, PaymentResponse, TransactionRequest, TransactionResponse, ChatMessage};

#[derive(Default)]
pub struct MyPaymentService;

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(&self, request: Request<PaymentRequest>) -> Result<Response<PaymentResponse>, Status> {
        let req = request.into_inner();
        println!("Received payment from {}: {}", req.user_id, req.amount);
        let reply = PaymentResponse { success: true };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MyTransactionService;

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type GetTransactionHistoryStream = tokio_stream::wrappers::ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {
        let user_id = request.into_inner().user_id;
        println!("Transaction history requested for {}", user_id);

        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            for i in 1..=5 {
                let response = TransactionResponse {
                    transaction_id: format!("tx_{}_{}", user_id, i),
                    status: "completed".into(),
                    amount: i as f64 * 10.0,
                    timestamp: format!("2025-05-08T12:00:0{}", i),
                };
                tx.send(Ok(response)).await.unwrap();
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

#[derive(Default)]
pub struct MyChatService;

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStream = tokio_stream::wrappers::ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(&self, request: Request<tonic::Streaming<ChatMessage>>) -> Result<Response<Self::ChatStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(msg) = inbound.message().await.unwrap() {
                println!("Received chat from {}: {}", msg.user_id, msg.message);
                let reply = ChatMessage {
                    user_id: "server".into(),
                    message: format!("Echo: {}", msg.message),
                };
                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Server listening on {}", addr);

    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();
    let chat_service = MyChatService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
} 