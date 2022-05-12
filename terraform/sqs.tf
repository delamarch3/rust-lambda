module "todo_queue" {
  source  = "terraform-aws-modules/sqs/aws"
  version = "~> 2.0"
  name = "todos"
}