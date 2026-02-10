use crate::parser;
use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;

pub fn send_request() -> Result<(), Box<dyn Error>>{
    let argument = parser::run_args()?; 

    let request = format!("GET {:?} HTTP/1.1\r\nHost: {}\r\n port: {:?}\r\nConnection: close\r\n\r\n", argument.host_str(), argument.path(), argument.port_or_known_default());
    println!("{}", &request);
    
    let  stream = TcpStream::connect(&request);
    
    let mut streamed_value = stream?;

    streamed_value.write_all(&request.as_bytes())?;
    
    let mut response = String::new();

   streamed_value.read_to_string(&mut response)?;

   println!("The server returns: {:?}", response);
    
    Ok(())
}
