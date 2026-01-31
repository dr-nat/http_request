use std::error::Error;
use std::env;


fn get_url() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/ URL were provided").into());
    }

    let _user_input = &args[1];

    Ok(())

}




fn main() {
    let result = get_url();

    match result {
        Ok(value) => value, 
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

}
