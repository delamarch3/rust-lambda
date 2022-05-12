module "dynamodb_table" {
  source   = "terraform-aws-modules/dynamodb-table/aws"

  name     = "todos"
  hash_key = "id"

  attributes = [
    {
      name = "id"
      type = "S"
    }
  ]
}