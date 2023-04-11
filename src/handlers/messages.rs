use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::models::message::{NewMessage, Message};

#[derive(Deserialize)]
pub struct SendMessageData {
    content: String,
    sender_id: i32,
    receiver_id: i32,
}

pub async fn send_message(message_data: web::Json<SendMessageData>) -> impl Responder {
    let new_message = NewMessage {
        content: message_data.content.clone(),
        sender_id: message_data.sender_id,
        receiver_id: message_data.receiver_id,
    };

    // TODO: Save the new_message to the database

    HttpResponse::Created().finish()
}

pub async fn get_messages(user_id: web::Path<i32>) -> impl Responder {
    // TODO: Fetch the messages for the user from the database

    HttpResponse::Ok().json(vec![
        Message {
            // Use dummy data for now
            id: 1,
            content: "Example message".to_string(),
            sender_id: 1,
            receiver_id: *user_id,
        }
    ])
}
