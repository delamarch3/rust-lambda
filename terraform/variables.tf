data "aws_api_gateway_rest_api" "todos_api" {
  depends_on = [
    aws_api_gateway_rest_api.main
  ]
  name = "todos-api"
}