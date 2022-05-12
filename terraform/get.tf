module "get" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "get-todo"
  handler       = "bootstrap"
  runtime       = "provided.al2"

  attach_policy_json = true
  policy_json        = file("./iam/policy.json")

  create_package         = false
  local_existing_package = "../functions/get_todo/target/lambda/bootstrap/bootstrap.zip"

  create_current_version_allowed_triggers = false
  allowed_triggers = {
    "APIGateway" = {
      service    = "apigateway"
      source_arn = "${data.aws_api_gateway_rest_api.todos_api.execution_arn}/*/*"
    }
  }
}
