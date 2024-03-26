# sys-compare
#### work in progress

### Create Snapshot
## ./sys-compare create [snapshot] [root_dir]
```shell
cargo run -- create ./test1.snap /home/foxx/Documents/
cargo run -- create ./test2.snap /home/foxx/Documents/
```

### Compare Snapshots
## ./sys-compare compare [.snap] [.snap] [created]|[deleted]|[changed]
```shell
cargo run -- compare ./test1.snap ./test2.snap created
```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
