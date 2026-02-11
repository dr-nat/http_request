use crate::parser;
use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;

pub fn send_request() -> Result<(), Box<dyn Error>>{
    let argument = parser::run_args()?; 

    let host = argument.host_str().ok_or("No host found")?;

    let port = argument.port_or_known_default().ok_or("Invalid port")?;

    let path = argument.path();
    
    let address = format!("{}:{}", host, port);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
   
    println!("\nConnecting to address: |{}|", address);
    let  stream = TcpStream::connect(&address);
    
    let mut streamed_value = stream?;

    streamed_value.write_all(&request.as_bytes())?;
    
    let mut response = String::new();

   streamed_value.read_to_string(&mut response)?;

   println!("The server returns: \n\n{:?}", response);
    
    Ok(())
}
