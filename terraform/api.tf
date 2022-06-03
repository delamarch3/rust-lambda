resource "aws_api_gateway_rest_api" "main" {
  name = "todos-api"
  body = file("api.yml")
}

resource "aws_api_gateway_deployment" "main" {
  rest_api_id = aws_api_gateway_rest_api.main.id

  triggers = {
    redeployment = sha1(jsonencode(aws_api_gateway_rest_api.main.body))
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "main" {
  deployment_id        = aws_api_gateway_deployment.main.id
  rest_api_id          = aws_api_gateway_rest_api.main.id
  stage_name           = "main"
}

output "apigw_address" {
  value = "${aws_api_gateway_deployment.main.invoke_url}main/"
}