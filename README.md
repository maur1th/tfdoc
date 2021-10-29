# tfdoc

This project aims at generating Terraform module documentation.

## Usage

tfdoc will parse all the files within a module's directory and generate markdown code accordingly:

```tf
# Title: The name of the module
# Top comment prefixed by "Title: " and the following lines
# will be at the top of the Markdown file

variable "environment" {
  description = "Variable descriptions will be parsed"
}

# tfdoc keeps comments right on top of resource, variable
# and output blocks. All variables and outputs are kept.
# Only resources with comments on top are.
resource "aws_instance" "this" {
  # stuff
}

resource "aws_instance" "no_comment_here" {
  # stuff
}

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
# The name of the module

Top comment prefixed by "Title: " and the following lines will be at the top of the Markdown file

## Resources

* `aws_instance.this`: tfdoc keeps comments right on top of resource, variable and output blocks. All variables and outputs are kept. Only resources with comments on top are.

## Inputs

* `environment`: Variable descriptions will be parsed

## Outputs

* `name`: We can have both comments on top and within outputs and variables
```

You can supply the `-t` parameter thusly `tfdoc -t $PATH_TO_MODULE` and get the output in table form rather than list form.

Original idea by <https://github.com/jyrivallan>
