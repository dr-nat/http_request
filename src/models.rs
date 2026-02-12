use std::collections::HashMap;


pub enum Methods {
    Get, 
    Post,
    Patch,
    Delete,
}


pub struct Request {
    method: Methods,
    host: String,
    path: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}
