use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

const TODO_TABLE: &str = "todos";

pub async fn update_todo(client: &Client, id: String, todo: String) -> Result<(), Error> {
    let id_av = AttributeValue::S(id);
    let todo_av = AttributeValue::S(todo);
    let request = client
        .update_item()
        .table_name(TODO_TABLE)
        .key("id", id_av)
        .expression_attribute_values(":v1", todo_av)
        .update_expression("SET todo = :v1");

    request.send().await?;

    Ok(())
}
