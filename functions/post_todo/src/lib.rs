use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use rand::{thread_rng, Rng};

const TODO_TABLE: &str = "todos";

pub struct TodoItem {
    pub id: String,
    pub todo: String,
}

impl TodoItem {
    pub fn new(todo: String) -> Self {
        let mut rng = thread_rng();
        let id: u32 = rng.gen();
        Self {
            id: id.to_string(),
            todo,
        }
    }

    pub async fn add(self, client: &Client) -> Result<(), Error> {
        let id_av = AttributeValue::S(self.id);
        let todo_av = AttributeValue::S(self.todo);
    
        let request = client
            .put_item()
            .table_name(TODO_TABLE)
            .item("id", id_av)
            .item("todo", todo_av);
    
        request.send().await?;
    
        Ok(())
    }
}
