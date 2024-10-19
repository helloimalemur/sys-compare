# sys-compare
Store Filesystem Integrity information via 'Snapshots' which contain a HashMap of the files and their corresponding hash signature from a specified directory.

## Installation
```shell
cargo install sys-compare
```

### Modes
```shell
Usage: sys-compare <COMMAND>

Commands:
  create   Create a snapshot
  compare  Compare two snapshots
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Create Snapshot
```shell
Usage: sys-compare create --root-dir <ROOT_DIR> --output-path <OUTPUT_PATH>

Options:
  -r, --root-dir <ROOT_DIR>        Directory to create snapshot from
  -o, --output-path <OUTPUT_PATH>  Snapshot output/save location
  -h, --help                       Print help
```

### Compare Snapshots
```shell
Usage: sys-compare compare [OPTIONS] --left <LEFT> --right <RIGHT>

Options:
  -l, --left <LEFT>            left side of diff
  -r, --right <RIGHT>          right side of diff
  -s, --selection <SELECTION>  OPTIONAL: specify which change type specifically to return
  -c <COUNT_ONLY>              OPTIONAL: when using selection specify to return count only or not [possible values: true, false]
  -h, --help                   Print help
```

## Example output
```shell
$ sys-compare create -r /etc -o ~/test.snapshot
Creating snapshot..
Total FileHash Entries 1891

$ sudo touch /etc/2

$ sys-compare create -r /etc -o ~/test2.snapshot
Creating snapshot..
Total FileHash Entries 1892

$ sys-compare compare -l ~/test.snapshot -r ~/test2.snapshot
Created: 1
Deleted: 0
Changed: 0

$ sys-compare compare -l ~/test.snapshot -r ~/test2.snapshot -s created
/etc/2
Created: 1

$ sys-compare compare -l ~/test.snapshot -r ~/test2.snapshot -s created -c
1
```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
