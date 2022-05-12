use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use get_todo::{get_todo_by_id, get_todos, TodoItem};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseBody {
    message: String,
    data: Option<Vec<TodoItem>>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    let (event, _context) = event.into_parts();
    let id = event.query_string_parameters.get("id");

    let config = aws_config::load_from_env().await;

    let ddb = Client::new(&config);

    if let Some(id) = id {
        match get_todo_by_id(&ddb, id).await {
            Ok(result) => {
                let res = ResponseBody {
                    message: "Success".to_owned(),
                    data: Some(result),
                    error: None,
                };
                let res = serde_json::to_string(&res).unwrap();

                Ok(ApiGatewayProxyResponse {
                    status_code: 200,
                    headers: HeaderMap::new(),
                    multi_value_headers: HeaderMap::new(),
                    body: Some(Body::Text(res)),
                    is_base64_encoded: Some(false),
                })
            }
            Err(e) => {
                let res = ResponseBody {
                    message: "Error".to_owned(),
                    data: None,
                    error: Some(format!("{}", e)),
                };
                let res = serde_json::to_string(&res).unwrap();
                Ok(ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: HeaderMap::new(),
                    multi_value_headers: HeaderMap::new(),
                    body: Some(Body::Text(res)),
                    is_base64_encoded: Some(false),
                })
            }
        }
    } else {
        match get_todos(&ddb).await {
            Ok(result) => {
                let res = ResponseBody {
                    message: "Success".to_owned(),
                    data: Some(result),
                    error: None,
                };
                let res = serde_json::to_string(&res).unwrap();

                Ok(ApiGatewayProxyResponse {
                    status_code: 200,
                    headers: HeaderMap::new(),
                    multi_value_headers: HeaderMap::new(),
                    body: Some(Body::Text(res)),
                    is_base64_encoded: Some(false),
                })
            }
            Err(e) => {
                let res = ResponseBody {
                    message: "Error".to_owned(),
                    data: None,
                    error: Some(format!("{}", e)),
                };
                let res = serde_json::to_string(&res).unwrap();
                Ok(ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: HeaderMap::new(),
                    multi_value_headers: HeaderMap::new(),
                    body: Some(Body::Text(res)),
                    is_base64_encoded: Some(false),
                })
            }
        }
    }
}