```shell
$ cargo run
   Compiling conways-rust-simulator v0.1.0 (/home/kane/conways-rust-simulator)
warning: unused variable: `y`
  --> src/main.rs:10:9
   |
10 |     for y in 0..3 {
   |         ^ help: if this is intentional, prefix it with an underscore: `_y`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `conways-rust-simulator` (bin "conways-rust-simulator") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/conways-rust-simulator`
============
1,1,1,
1,1,1,
1,1,1,

1,0,1,
0,0,0,
1,0,1,

============
============
1,0,1,
0,0,0,
1,0,1,

0,0,0,
0,0,0,
0,0,0,

============
============
0,0,0,
0,0,0,
0,0,0,

0,0,0,
0,0,0,
0,0,0,

============
```
