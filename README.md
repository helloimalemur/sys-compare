# sys-compare
Check Filesystem Integrity via "Snapshots" containing hash of all files within specified directories.

### Modes
```shell
Usage: sys-compare <COMMAND>

Commands:
  create   
  compare  
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Create Snapshot
```shell
Usage: sys-compare create --root-dir <ROOT_DIR> --output-path <OUTPUT_PATH>

Options:
  -r, --root-dir <ROOT_DIR>        
  -o, --output-path <OUTPUT_PATH>  
  -h, --help                       Print help
```

### Compare Snapshots
```shell
Usage: sys-compare compare [OPTIONS] --left <LEFT> --right <RIGHT>

Options:
  -l, --left <LEFT>            
  -r, --right <RIGHT>          
  -s, --selection <SELECTION>  
  -h, --help                   Print help
```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
