use std::error::Error;
use std::env;


pub fn get_url() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/ URL were provided").into());
    }

    let user_input = args[1].clone();

    Ok(user_input)

}


