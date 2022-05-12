use aws_sdk_sqs::{Client, Error};
use std::env;

const TODO_QUEUE: &str = "TODO_QUEUE";

pub async fn send_message(client: &Client, message: String) -> Result<(), Error> {
    let url = env::var(TODO_QUEUE).expect("Environment variable not set");

    let request = client
        .send_message()
        .queue_url(url)
        .message_body(message);

    request.send().await?;

    Ok(())
}
