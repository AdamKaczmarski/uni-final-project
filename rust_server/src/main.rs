use rust_server::run;
use std::env;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //Quick check if some parameters were provided the program will panic if the first parameter is
    //not a port number
    let mut addr: String = String::from("0.0.0.0:");
    if args.len() > 1 {
        addr.push_str(&args[1]);
    } else {
        //if runtime argument was not provided the program will try to use the environment variable
        //called RS_SERVER_PORT and use its value if such value does not exist or cannot be parsed
        //the program will default to port 8081
        let env_port = env::var("RS_SERVER_PORT").unwrap_or("8081".to_string());
        addr.push_str(&env_port);
    }
    let listener = TcpListener::bind(addr).expect("Failed to bind the address");
    run(listener)?.await
}
