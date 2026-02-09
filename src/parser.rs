use std::error::Error;
use crate::cli_args;
use url::*;

pub fn parse_args() -> Result<Url, Box<dyn Error>>{
    let args = cli_args::get_url(); 

    let url = Url::parse(&args?.as_str())?;

    Ok(url)
}
