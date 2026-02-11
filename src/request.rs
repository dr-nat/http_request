use rustls::{ClientConfig, ClientConnection, Stream};
use webpki_roots;
use std::sync::Arc;
use crate::parser;
use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;

pub fn send_request() -> Result<(), Box<dyn Error>>{

    let argument = parser::run_args()?;

    if argument.scheme() == "http" {

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
   } else if argument.scheme() == "https" {

        let host = argument.host_str().ok_or("No host found")?.to_string();

        let port = argument.port_or_known_default().ok_or("Invalid port")?;

        let path = argument.path();
        
        let address = format!("{}:{}", host, port);

        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
       
        println!("\nConnecting to address: |{}|", address);


       let mut response_bytes = Vec::new();
       let mut root_store = rustls::RootCertStore::empty();

       root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

       let config = Arc::new(
            ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
       );

       let server_name: rustls::pki_types::ServerName = host.try_into()
            .map_err(|_| "Invalid DNS Name")?;

       let mut client = ClientConnection::new(config, server_name)?;

       let mut tcp = TcpStream::connect(&address)?;

       {
           let mut tls_stream = Stream::new(&mut client, &mut tcp);

           tls_stream.write_all(request.as_bytes())?;
           tls_stream.read_to_end(&mut response_bytes)?;
       }
       let response = String::from_utf8_lossy(&response_bytes).to_string();
            
       if let Some((headers, body)) = response.split_once("\r\n\r\n") {
            println!("\n --- HEADERS ---");
            println!("\n {:?}", headers);

            println!("\n --- BODY --- ");
            println!("\n {:?}", body);

            if let Some(status_code) = headers.lines().next() {
                let pattern: Vec<&str> = status_code.split_whitespace().collect();

                if let Some(code) = pattern.get(1) {
                    match *code {
                        "200" => println!("\n\x1b[32m Status: Successful ({})\x1b[0m", code),
                        "400" => println!("\n\x1b[31m Status: Not found {}\x1b[0m", code),
                        _ => println!("Status code: {}", code)
                    }
                }
            }
       } else {
            println!("The server reeturns: \n\n{:?}", response);
    
       }
   }
    Ok(())
}
