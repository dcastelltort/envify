# Envify

usually you want you app to rely on environment variable for configuration, and not rely on config files that need to be deployed specifically to your environment (development, staging, release), etc. Thus you end up with different ways to create those env var while you develop , or you deploy in production.

The tool generates a sh file to declare environment variable or a salt file from a json file 

This way you maintain json file, but use the tool to generate the correct file for your need to declare those env var.

## TODO:
* support uppercase output format enum flag on the cli (bash, salt, more ?)
* support prefix flag on the cli
* generate the file
* support output file name
* what to do with arrays ?
