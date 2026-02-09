use std::error::Error;
use crate::cli_args;
use url::*;

pub fn parse_args(input: &str) -> Result<Url, Box<dyn Error>> {


    let url = Url::parse(input)?;
    
    let _host = &url.host();

    let _path = &url.path();

    let _port = &url.port_or_known_default();

    Ok(url)
}

pub fn run_args() -> Result<Url, Box<dyn Error>> {
    let link = cli_args::get_url()?;

    parse_args(&link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {

        let link = "https://arstechnica.com";
        
        let result = parse_args(link);

        assert!(result.is_ok());

        let url = result.unwrap();

        assert_eq!(url.host_str(), Some("arstechnica.com"));
        assert_eq!(url.port_or_known_default(), Some(443))
    }
}
