module "patch" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "patch-todo-enqueue"
  handler       = "bootstrap"
  runtime       = "provided.al2"

  attach_policy_json = true
  policy_json        = file("./iam/policy.json")


  create_package         = false
  local_existing_package = "../functions/patch_todo_enqueue/target/lambda/bootstrap/bootstrap.zip"

  create_current_version_allowed_triggers = false
  allowed_triggers = {
    "APIGateway" = {
      service    = "apigateway"
      source_arn = "${data.aws_api_gateway_rest_api.todos_api.execution_arn}/*/*"
    }
  }

  environment_variables = {
    "TODO_QUEUE" = data.aws_sqs_queue.todos.url
  }
}

module "patch_enqueue" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "patch-todo-dequeue"
  handler       = "bootstrap"
  runtime       = "provided.al2"

  attach_policy_json = true
  policy_json        = file("./iam/policy.json")


  create_package         = false
  local_existing_package = "../functions/patch_todo_dequeue/target/lambda/bootstrap/bootstrap.zip"

  event_source_mapping = {
    sqs = {
      event_source_arn = data.aws_sqs_queue.todos.arn
    }
  }
}

data "aws_sqs_queue" "todos" {
  depends_on = [
    module.todo_queue
  ]
  name = "todos"
}