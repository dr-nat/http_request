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

    // The idead behind connecting to the hosts server!
    let  stream = TcpStream::connect(&address);
    
    // Unpacking the successful scenario of the connection!
    let mut streamed_value = stream?;

    // The process of sending the entire address to the server!
    streamed_value.write_all(&request.as_bytes())?;


   // The process of storing the response in form of a buffer! 
    let mut response = String::new();

   streamed_value.read_to_string(&mut response)?;

   if let Some((headers, body)) = response.split_once("\r\n\r\n") {
        println!(" --- HEADERS --- ");
        println!("{}", headers);

        println!("\n--- BODY ---");
        println!("{}", body);

        if let Some(status_line) = headers.lines().next() {
            let patterns: Vec<&str> = status_line.split_whitespace().collect();

            if let Some(code) = patterns.get(1) {
                match *code {
                    "200" => println!("\x1b[32mStatus: Success ({})\x1b[0m", code),
                    "404" => println!("\x1b[31mStatus: Not Found ({})\x1b[0m", code),
                    _ => println!("Status Code: {}", code),
                }
            }
        }
   } else {
   
   println!("The server returns: \n\n{:?}", response);
   
   }
    Ok(())
}
