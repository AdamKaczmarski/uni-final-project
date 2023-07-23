# How to build and run the rust_server locally
Prerequisites
- Installed Rust toolchain for version 1.68 

1. Build the binary executable for Linux, macOS and Unix-like systems. 
```
cargo build --release
```
The executable artifact will be located in `./target/release` directory under the name of `rust_server`

1. Build the binary executable for 64-bit Windows platform.
```
cargo build --target x86_64-pc-windows-gnu --release
```
The executable artifact will be located in `./target/x86_64-pc-windows-gnu/release` directory under the name of `rust_server.exe`

2. Run 2 rust_servers, one that acts as rust_server and the second one that acts as rust_server_provider 
```
./target/release/rust_server 8081
./target/release/rust_server 8082
```

One could change the port numbers when launching the server using
```
./target/release/rust_server <PORT>
```
**It is recommended** to run 2 Rust servers that utilize ports 8081 and 8082.\
One will behave as rust-server, the second will be the rust-server-provider.\
The benchmarking script is coded to use those ports in its requests.\
One could change the aforementioned ports but will have to adjust it in the 
[benchmark-localhost.py](../server-benchmark/benchmark-localhost.py). 


# How to build and run Docker image of the rust_server

Prerequisites:
- Docker Engine (v20.10.24)

The action is automated when using docker compose in the root directory of the project. See [README.md](../README.md)

1. In terminal run:
```
docker image build . -t rust-server
```
2. To spawn up the containers run:
```
docker container run -d -p 8081:8081 -e RS_SERVER_PORT=8081 --name rust-server rust-server
docker container run -d -p 8082:8082 -e RS_SERVER_PORT=8082 --name rust-server-provider rust-server
