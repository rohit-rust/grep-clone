
use std::{error::Error, fs::File, io::{self, Read}};

use thiserror::Error;


#[derive(Error,Debug)]
pub enum CustomError {
    #[error("{0}")]
    Message(String)
}


pub struct Input {
    pub data: String,
    pub path: String,
    pub ignore_case: bool
}

impl Input {
    pub fn build(args: &Vec<String>,ignore_case: bool) -> Result<Self,CustomError> {
        if args.len() < 3 {
            return Err(CustomError::Message("expected at least two arguments".to_string()))
        }

        Ok(Self {
            data: args[1].to_owned(),
            path: args[2].to_owned(),
            ignore_case
        })
    }
}



pub fn read_data(config: &Input) -> Result<String,Box<dyn Error>> {
    let mut data = String::new();
    let mut file_exists = File::open(&config.path).unwrap_or_else(|error|{
        if error.kind() == io::ErrorKind::NotFound {
            panic!("file does not exists")
        } else if error.kind() == io::ErrorKind::PermissionDenied {
            panic!("file does not have permission")
        } else {
            panic!("{}",io::Error::new(io::ErrorKind::Other, "unknown error occurred !"))
        }
    });

    file_exists.read_to_string(&mut data).unwrap();
    Ok(data)
}
