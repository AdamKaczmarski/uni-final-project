# Individual Project 2023

## Folder structure

Each directory has its own README file about the deployment of the component for the cleanliness and ease of use.

- [java-server](./java-server/) - Contains Java files, maven configuration file [pom.xml](./java-server/pom.xml), [Dockerfile](./java-server/Dockerfile) and bundled Java Archive [java-server.jar](./java-server/java-server.jar) that's ready to run.   
Build and run instructions: [JAVA_README](./java-server/JAVA_README.md)
- [rust_server](./rust_server/) - Contains Rust source files, manifest file [Cargo.toml](./rust_server/Cargo.toml), [Dockerfile](./rust_server/Dockerfile) and compiled executable.
There are 2 versions of the binary file, one that can be ran on Windows [rust_server.exe](./rust_server/rust_server.exe),
second that's for Linux, macOS and Unix-like systems [rust_server](./rust_server/rust_server).   
Build and run instructions: [RUST_README](./rust_server/RUST_README.md)
- [typescript-client](./typescript-client/) - Contains TypeScript files, configuration files (node, vite and TypeScript), Apache2 config files, [Dockerfile](./typescript-client/Dockerfile) and bundled static files that are ready to be deployed on a web server
in [dist/](./typescript-client/dist/) directory.   
 Build and run instructions: [TS_README](./typescript-client/TS_README.md)
- [server-benchmark](./server-benchmark/) - Contains a python script that benchmarks the parsing time between java and rust servers. There are 2 versions of the file dependend on the path that one chose to run the environment.
[benchmark-dockerized.py](./server-benchmark/benchmark-dockerized.py) is to be used when servers are running inside a container, [benchmark-localhost.py](./server-benchmark/benchmark-localhost.py) should be used when the application are ran locally.   
 Build and run instructions: [SB_README](./server-benchmark/SB_README.md)
- [ui-benchmarking](./ui-benchmarking/) - Contains nodeJS Pupeteer script that behaves as a synthetic client launching Google Chrome browser and performing actions on the specified site. 
Additionally, there is a python script that generates graphs from the csv files found in the directory. There are 2 versions of the nodeJS script [ui-benchmark-dockerized.js](./ui-benchmarking/ui-benchmark-dockerized.js) is to be used
when servers are running inside Docker containers, [ui-benchmark-localhost.js](./ui-benchmarking/ui-benchmark-localhost.js) should be used when servers are launched locally using the provided artifacts.   
Build and run instructions: [UIB_README](./ui-benchmarking/UIB_README.md)

I **recommend to use Docker** to deploy the servers. The steps are very minimal and automated, Dockerfiles in specified directories handle building and create the necessary images.

## How to run using Docker
Prerequisites:
- Docker Engine version 20
1. On the root directory of this project run
```
docker compose build && docker compose up -d
```
Docker will build rust-server, java-server and typescript-client images and run 5 containers.
- rust-server (port 8081)
- rust-server-provider (port 8082)
- java-server (port 8080)
- java-server-provider (port 8079)
- typescript-client (port 8060)

Those servers will be accessible by the host on ports specified in the [docker-compose.yml](./docker-compose.yml).  
The directories [java-server](./java-server/), [rust_server](./rust_server/) and [typescript-client](./typescript-client/)
have Dockerfiles inside of them that build the necessary artifacts and create the ready-to-deploy Docker images.

2. Benchmarking between Java and Rust

The script to benchmark JSON and BSON between Java and Rust is available in directory [server-benchmark](./server-benchmark/).  
The instruction how to run the script and what does it output is available in that directory -> [SB_README.md](./server-benchmark/SB_README.md).

3. Benchmarking the typescript-client.

The script to benchmark JSON and BSON from both Java and Rust is available in directory [ui-benchmarking](./ui-benchmarking/).  
The instruction how to run the script and what does it output is available in that directory -> [UIB_README.md](./ui-benchmarking/UIB_README.md).

## How to run locally

The project requires 4 running web server backends and 1 Apache2 web server.
- rust-server (port 8081)
- rust-server-provider (port 8082)
- java-server (port 8080)
- java-server-provider (port 8079)
- typescript-client (port 80) \[Apache2 web server\]

1. Launching the servers

To launch Java servers, please read [Java build and run instructions](./java-server/JAVA_README.md).  
To launch Rust servers, please read [Rust build and run instructions](./rust_server/RUST_README.md).  
To launch the Apache2 web server, please read [Typescript-client build and run instructions](./typescript-client/TS_README.md).

2. Benchmarking between Java and Rust

The script to benchmark JSON and BSON between Java and Rust is available in directory [server-benchmark](./server-benchmark/).  
The instruction how to run the script and what does it output is available in that directory -> [SB_README.md](./server-benchmark/SB_README.md).

3. Benchmarking the typescript-client.

The script to benchmark JSON and BSON from both Java and Rust is available in directory [ui-benchmarking](./ui-benchmarking/).  
The instruction how to run the script and what does it output is available in that directory -> [UIB_README.md](./ui-benchmarking/UIB_README.md).
