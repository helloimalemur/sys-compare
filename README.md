# sys-compare

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
