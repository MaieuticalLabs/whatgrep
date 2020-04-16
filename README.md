# Whatgrep
Simple cli program to search for lines in a given language


## Usage
`$ cat myfile.txt | whatgrep -l ita -p ita eng -`


### Available options

```
Whatgrep 0.1.0
Search lines for a given language

USAGE:
    whatgrep [FLAGS] [OPTIONS] [--] [input]

FLAGS:
    -h, --help            Prints help information
    -v, --invert_match    Select non-matching lines
    -V, --version         Prints version information

OPTIONS:
    -l, --language <language>      Target language [default: eng]
    -p, --pool <pool>...           Whitelist of possible languages (recommended for short texts)
    -t, --threshold <threshold>    from 0.0 to 1.0 confidence threshold [default: 0.6]

ARGS:
    <input>    Input file or stdin
```


