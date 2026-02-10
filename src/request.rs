use crate::parser;
use std::error::Error;

pub fn send_request() -> Result<(), Box<dyn Error>>{
    let request = parser::run_args()?; 

    let _ = format!("GET {:?} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", request.host_str(), request.path());
    
    
    Ok(())
}
