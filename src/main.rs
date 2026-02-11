use http_request::request::send_request;
use http_request::cli_args::get_url;

fn main() {
    let result = get_url();

    match result {
        Ok(_) => {}, 
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let response = send_request();
    
    match response {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(2);
        }
    }
}
