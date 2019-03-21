# tfdoc

This project aims at generating Terraform module documentation.

## Usage

tfdoc will parse all the files within a module's directory and generate a README.tf accordingly:

```tf
# Title: This is a module title
# Top comment prefixed by "Title: " and the following lines
# will be at the top of the Markdown file

variable "environment" {
  description = "Variable descriptions will be parsed"
}

# tfdoc keeps comments right on top of resource, variable
# and output blocks
resource "aws_instance" "this" {}

##
## tfdoc discards other "orphaned" comments
##

# We can have both comments on top
output "name" {
  description = "and within outputs and variables"
}

# Data blocks are ignored
data "aws_ami" "node" {}
```

```sh
$ tfdoc $PATH_TO_MODULE
# This is a random title

Top comment prefixed by "Title: " and the following lines will be at the top of the Markdown file

## Inputs

* `environment`: Variable descriptions will be parsed

## Outputs

* `name`: We can have both comments on top and within outputs and variables
```

Original idea by https://github.com/jyrivallan
