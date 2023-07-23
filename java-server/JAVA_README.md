
# How to build and run locally
Prerequisites:
- Java 17

1. To build the project, in the terminal execute:
```
./mvnw dependency:resolve
./mvnw package -Pdefault
```
2. To run 2 servers using G1GC
It is recommended to specify the memory allocation for the JVM.\
It will reduce the number of miliseconds spend in GarbageCollection during the execution.
```
java -Xms1024m -Xmx1024m -XX:+UseG1GC -jar ./target/java-server-1.0.0.jar --server.port=8079
java -Xms1024m -Xmx1024m -XX:+UseG1GC -jar ./target/java-server-1.0.0.jar --server.port=8080
```
Otherwise the `Xms` and `Xmx` parameters can be omitted. The benchmarks were ran using the commands specified above.
```
java -XX:+UseG1GC -jar ./target/java-server-1.0.0.jar --server.port=8079
java -XX:+UseG1GC -jar ./target/java-server-1.0.0.jar --server.port=8080
```
It is also possible to specify the server port.
```
java -Xms1024m -Xmx1024m -XX:+UseG1GC -jar ./target/java-server-1.0.0.jar --server.port=<PORT>
```
**It is recommended** to run 2 Java servers that utilize ports 8079 and 8080.\
One will behave as java-server, the second will be the java-server-provider.\
The benchmarking script is coded to use those ports in its requests.\
One could change the aforementioned ports but will have to adjust it in the 
[benchmark-localhost.py](../server-benchmark/benchmark-localhost.py).

# How to build a Docker image

Prerequisites:
- Docker Engine (v20.10.24)

The action is automated when using docker compose in the root directory of the project. See [README.md](../README.md)

1. In terminal run:
```
docker image build . -t java-server
```
2. To spawn up a container run:
```
docker container run -d -p 8080:8080 -e JV_SERVER_PORT=8080 --name java-server java-server
docker container run -d -p 8079:8079 -e JV_SERVER_PORT=8079 --name java-server-provider java-server
```
