# Envify

Usually you want your app to rely on environment variables for configuration, and not rely on config files that need to be deployed specifically to your environment (development, staging, release), etc. Thus, depending on your environment, you might end up with different ways to create those env vars while you develop, or you deploy in production.

The tool generates a shell file to declare environment variable or a salt file from a json file 

That way, you maintain only a json file, but use the tool to generate the appropriate file for your need to declare those env var.

## Help
```
USAGE:
    envify [FLAGS] [OPTIONS] <file> <SUBCOMMAND>

FLAGS:
    -h, --help         Prints help information
    -u, --uppercase    uppercase the keys found in the json input
    -V, --version      Prints version information
    -v, --verbose      Pass many times for more log output

OPTIONS:
    -o, --output <output_file>    name of the file with extension, otherwise it uses input with appropriate extension
    -p, --prefix <prefix>         prefix variable name using the one provided

ARGS:
    <file>    The file to read

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    salt     generate a salt file
    shell    generate a shell file
````

## TODO:
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
'envify examples/test.json -p tag -u shell' gives this shell:
```
#!/bin/sh
export TAG_DABOOL=1
export TAG_DAFLOAT=1.0
export TAG_DANUM=1000
export TAG_ROOT_SUB1=0
export TAG_ROOT_SUB2="this is a test"
export TAG_ROOTKEY="value"
```
'envify examples/test.json salt' generates this salt

```
salt '*' environ.setval DABOOL 1
salt '*' environ.setval DAFLOAT 1.0
salt '*' environ.setval DANUM 1000
salt '*' environ.setval ROOT_SUB1 0
salt '*' environ.setval ROOT_SUB2 "this is a test"
salt '*' environ.setval ROOTKEY "value"
```