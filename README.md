# Whatgrep
Simple cli program to search for lines in a given language


## Usage
`$ cat myfile.txt | whatgrep -l ita -p ita,eng -`


### Available options

```
whatgrep 0.1.0
Search lines for a given language

USAGE:
    whatgrep [OPTIONS] <input>

ARGS:
    <input>    input file ("-" for stdin)

OPTIONS:
    -h, --help                     Print help information
    -l, --language <language>      target language [default: eng]
    -p, --pool <pool>...           command-separated list of possible languages
    -t, --threshold <threshold>    from 0.0 to 1.0 confidence threshold [default: 0.6]
    -v, --invert_match             select non-matching lines
    -V, --version                  Print version information
```
