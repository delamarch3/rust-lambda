use aws_lambda_events::event::sqs::SqsEvent;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::{Deserialize};
use patch_todo_dequeue::update_todo;

#[derive(Deserialize)]
struct MessageBody {
    id: String,
    todo: String
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(
    event: LambdaEvent<SqsEvent>,
) -> Result<(), LambdaError> {
    let (event, _context) = event.into_parts();
    let event_body = &event.records[0].body.as_ref().unwrap();
    let message: MessageBody = serde_json::from_str(event_body.to_owned())?;

    let config = aws_config::load_from_env().await;

    let ddb = Client::new(&config);

    update_todo(&ddb, message.id, message.todo).await?;

    Ok(())
}
