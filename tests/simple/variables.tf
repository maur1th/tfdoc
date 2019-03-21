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
