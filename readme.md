# wool
Extensible [grip](https://github.com/joeyespo/grip) clone

## Installation 

`cargo install wool` 

## Usage
```
USAGE:
    wool [FLAGS] <infile> [outfile]

FLAGS:
    -b, --no-browser          Don't open browser (browser opens by default)
    -d, --d2                  Render inline D2 diagrams (requires d2 command)
    -e, --export              Export html
    -h, --help                Prints help information
    -s, --highlight           Syntax highlighting
    -k, --katex               Include katex in rendering
    -n, --no-preview-frame    Don't render the preview frame
    -V, --version             Prints version information 
    
ARGS:
    <infile>     Sets the input file to use
    <outfile>    Sets the output file to use
```

#### Example

preview on localhost (opens browser automatically):
`wool readme.md`

preview without opening browser:
`wool readme.md --no-browser`

export to html:
`wool readme.md --export mypreview.html`

equations with katex:
`wool readme.md -k`

diagrams with D2 (requires [d2](https://d2lang.com) to be installed):
`wool readme.md -d`

###### Experimental

Syntax highlighting:
`wool readme.md -s`

<!--
## Installation Options

Cargo: 
`cargo install wool`

Arch: 
`pacman -S wool`
-->
