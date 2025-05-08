pub mod grpc_tutorial {
    tonic::include_proto!("grpc_tutorial");
}

use tonic::Request;
use grpc_tutorial::payment_service_client::PaymentServiceClient;
use grpc_tutorial::transaction_service_client::TransactionServiceClient;
use grpc_tutorial::chat_service_client::ChatServiceClient;
use grpc_tutorial::{PaymentRequest, TransactionRequest, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Unary call: ProcessPayment
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let request = Request::new(PaymentRequest { user_id: "user1".into(), amount: 123.45 });
    let response = payment_client.process_payment(request).await?;
    println!("Payment response: {:?}", response.into_inner());

    // Server streaming call: GetTransactionHistory
    let mut tx_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    let mut stream = tx_client
        .get_transaction_history(Request::new(TransactionRequest { user_id: "user1".into() }))
        .await? 
        .into_inner();
    println!("Transaction history:");
    while let Some(tx) = stream.message().await? {
        println!("- {:?}", tx);
    }

    // Bi-directional streaming call: Chat
    let mut chat_client = ChatServiceClient::connect("http://[::1]:50051").await?;
    let (mut tx, mut rx) = chat_client.chat().await?.into_parts();

    // Spawn a task to send messages
    tokio::spawn(async move {
        let messages = vec![
            ChatMessage { user_id: "user1".into(), message: "Hello".into() },
            ChatMessage { user_id: "user1".into(), message: "How are you?".into() },
            ChatMessage { user_id: "user1".into(), message: "Bye".into() },
        ];
        for msg in messages {
            tx.send(msg).await.unwrap();
        }
        tx.close().await.unwrap();
    });

    println!("Chat replies:");
    while let Some(reply) = rx.message().await? {
        println!("* {:?}", reply);
    }

    Ok(())
} 