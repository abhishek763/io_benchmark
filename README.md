## io_benchmark

- For compiling
```cargo build --release```

- For benchmarking async reading
```./target/release/async-io -i <inp file> -b <buf size> -n <num of parallel reads> -r -a```
  
- For benchmarking async writing
```
./target/release/async-io -o 5G -b <buf size> -w
```

- For benchmarking sync reading
```./target/release/sync-io -i <inp file> -b <buf size> -r```
  
- For benchmarking sync writing
```
./target/release/async-io -o 5G -b <buf size> -w
```

- run.py is just a wrapper python script to run benchmarks
