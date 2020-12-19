extern crate tiny_http;
extern crate clap;
extern crate wmi;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tiny_http::{Response, Server};
use wmi::{COMLibrary, Variant, WMIConnection,WMIError};
use clap::{Arg, App};

#[derive(Serialize, Deserialize)]
struct Request{
    namespace:String,
    query:String,
}

fn main() {
    let (address,port) = init_flags();
    println!("API Running on: {}:{}", address, port);

    // Creating server with address and port variables
    let server = match Server::http(format!("{}:{}",address,port)){
        Ok(conn) => conn,
        Err(msg) => {
            println!("Can not create HTTP Listener. Error: {}",msg);
            return;
        }
    };

    // Creating loop for handling requests
    // Every request process with 1 thread
    // So at the same time we can process only one request
    loop {
        // Getting request object from server.recv()
        let mut request = match server.recv(){
            Ok(request) => request,
            Err(msg) => {
                println!("Receiving Request failed : {}", msg);
                continue;
            }
        };

        // Request body converting to Request Struct
        let mut content = String::new();
        match request.as_reader().read_to_string(&mut content){
            Ok(_) => {},
            Err(msg) => {
                println!("{}",msg);
                continue;
            }
        };
        
        let req: Request = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => {
                let response = Response::from_string("{\"error\":\"Json Decode Error\"}");
                request.respond(response).unwrap();
                continue;
            },
        };

        // Querying from WMI and response json
        let json: String = match query_on_wmi(&req.namespace,req.query){
            Ok(json) => json,
            Err(_) => {
                let response = Response::from_string("{\"error\":\"Query Error\"}");
                request.respond(response).unwrap();
                continue;
            },
        };
        let mut response = Response::from_string(json);
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
        response.add_header( header );
        request.respond(response).unwrap();
    }
}

// init_flags func will create description for app's CLI.
// Takes 2 arg named address and port.
// This args defined 127.0.0.1:8080 by default.
fn init_flags()-> (String,String){
    let matches = App::new("Rust Rest-Wmi API")
    .version("0.1.0")
    .author("Yusuf Turhan Papurcu <yusufturhanp@gmail.com>")
    .about("Windows WMI to REST API Conventer")
    .arg(Arg::with_name("port")
             .long("port")
             .takes_value(true)
             .help("Port for running API"))
    .arg(Arg::with_name("address")
             .long("address")
             .takes_value(true)
             .help("Address for running API"))
    .get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let address = matches.value_of("address").unwrap_or("127.0.0.1");

    (address.to_owned(),port.to_owned())
}

// query_on_wmi for query wmi in machine
fn query_on_wmi(namespace :&str,query :String)->Result<String,WMIError>{
    
    // Creating new COM Port
    let com_con = match COMLibrary::new(){
        Ok(res) => res,
        Err(msg) => {
            println!("Error: {}",msg);
            return Err(msg)
        },
    };

    // Create new WMI Connection using COM Port
    // This connection only works in given namespace
    let wmi_con = match WMIConnection::with_namespace_path(namespace,com_con.into()){
        Ok(res) => res,
        Err(msg) => {
            println!("Error: {}",msg);
            return Err(msg)
        },
    };

    // Do Query 
    let results: Vec<HashMap<String, Variant>> = match wmi_con.raw_query(query){
        Ok(data) => data,
        Err(msg) => {
            println!("Error: {}",msg);
            return Err(msg)
        },
    };

    // Convert result to json and return it
    let json = match serde_json::to_string(&results){
        Ok(v) => v,
        Err(_) => {
            return Err(WMIError::SerdeError("Json Parse Error".to_string()))
        },
    };
    Ok(json)
}