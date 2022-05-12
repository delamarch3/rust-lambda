use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};

const TODO_TABLE: &str = "todos";

pub async fn delete_todo(client: &Client, id: String) -> Result<(), Error> {
    let id_av = AttributeValue::S(id);

    let request = client.delete_item().table_name(TODO_TABLE).key("id", id_av);
    request.send().await?;

    Ok(())
}
