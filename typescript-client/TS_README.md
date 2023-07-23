# How to build and run typescript-client locally

Prerequisites
- Node 18
- npm 9.5
- Apache2 version 2.4 

1. Install the dependencies via npm
```
npm install
```
2. Create the static files. They can be found in `dist/` directory after the script finishes.
```
npm run build
```
3. Move the files from `dist/` to `/var/www/html`. Apache2 will serve the files from this directory.

4. Copy the config files from `./apache_config/` to `/etc/apache2/` (or other directory of your Apache2 configuration files).

The configuration files contain routing to the rust and java servers.

5. Start the Apache2 server

## Adjust the UI files to omit the requirement of Apache2

This method will allow to run the dev build using vite.
1. Install the dependencies via npm
```
npm install
```
2. Adjust the [`src/App.tsx`](./src/App.tsx)
Change the values of 
```typescript
    let java_server_local = "/java";
    let rust_server_local = "/rust";
```
to URLs of the java-server and rust-server. \
If ports are set by the instructions the values should be (they are also commented in the code, one could just recomment the lines in the code)
```typescript
    let java_server_local = "http://localhost:8080";
    let rust_server_local = "http://localhost:8081";
```
3. Run the dev mode
By default vite runs its dev server on [http://localhost:5173](http://localhost:5173)
```
npm run dev
```
 
# How to build and run Docker image of the typescript-client 

Prerequisites:
- Docker Engine (v20.10.24)

The action is automated when using docker compose in the root directory of the project. See [README.md](../README.md)

1. In terminal run:
```
docker image build . -t typescript_client
```
2. To spawn up the containers run:
```
docker container run -d -p 8060:80 -e JAVA_PORT=8080 -e RUST_PORT=8081 -e JAVA_SERVER=java-server -e RUST_SERVER=rust-server -e APACHE_LISTEN_PORT=80 -name typescript-client typescript_client
```
