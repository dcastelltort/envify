# Envify

(THIS IS STILL WIP AND UNDER HEAVY DEV)

Usually you want you app to rely on environment variable for configuration, and not rely on config files that need to be deployed specifically to your environment (development, staging, release), etc. Thus you end up with different ways to create those env var while you develop , or you deploy in production.

The tool generates a sh file to declare environment variable or a salt file from a json file 

This way you maintain json file, but use the tool to generate the correct file for your need to declare those env var.

## Help
```
USAGE:
    envify [FLAGS] [OPTIONS] <file> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Pass many times for more log output

OPTIONS:
    -o, --output <output_file>    name of the file with extension, otherwise it uses input with appropriate extension

ARGS:
    <file>    The file to read

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    salt     generate a salt file
    shell    generate a shell file
````

## TODO:
* support prefix flag on the cli
* what to do with arrays ?

## Examples

This json
```
{
    "root" : {
        "sub1" : 0,
        "sub2" : "this is a test"
    },
    "rootkey" : "value",
    "dabool" : 1,
    "danum" : 1000,
    "dafloat" : 1.0
}
```
gives this shell:
```
#!/bin/sh
export DABOOL=1
export DAFLOAT=1.0
export DANUM=1000
export ROOT_SUB1=0
export ROOT_SUB2="this is a test"
export ROOTKEY="value"
```

or this salt:
```
salt '*' environ.setval DABOOL 1
salt '*' environ.setval DAFLOAT 1.0
salt '*' environ.setval DANUM 1000
salt '*' environ.setval ROOT_SUB1 0
salt '*' environ.setval ROOT_SUB2 "this is a test"
salt '*' environ.setval ROOTKEY "value"
```