use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;

const TODO_TABLE: &str = "todos";

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    id: String,
    todo: String,
}

pub async fn get_todos(client: &Client) -> Result<Vec<TodoItem>, Error> {
    let request = client.scan().table_name(TODO_TABLE);

    let response = request.send().await?;

    let items: Vec<TodoItem> = if let Some(items) = response.items {
        from_items(items).unwrap()
    } else {
        vec![]
    };

    Ok(items)
}

pub async fn get_todo_by_id(client: &Client, id: &String) -> Result<Vec<TodoItem>, Error> {
    let id_av = AttributeValue::S(id.to_owned());
    let request = client
        .query()
        .table_name(TODO_TABLE)
        .key_condition_expression("id = :v1")
        .expression_attribute_values(":v1", id_av);

    let response = request.send().await?;

    let items: Vec<TodoItem> = if let Some(items) = response.items {
        from_items(items).unwrap()
    } else {
        vec![]
    };

    Ok(items)
}
