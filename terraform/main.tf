resource "null_resource" "build" {
  provisioner "local-exec" {
    command = "./build_all.sh"
  }

  provisioner "local-exec" {
    when    = destroy
    command = "./clean_all.sh"
  }
}
