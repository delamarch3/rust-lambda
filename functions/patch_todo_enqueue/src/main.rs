use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_sqs::Client;
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Serialize;
use patch_todo_enqueue::send_message;

#[derive(Serialize)]
struct ResponseBody {
    message: String,
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
    let event_body = event.body.unwrap();
    let config = aws_config::load_from_env().await;

    let sqs = Client::new(&config);


    match send_message(&sqs, event_body).await {
        Ok(_) => {
            let res = ResponseBody {
                message: "Success".to_owned(),
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
