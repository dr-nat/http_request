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

}
