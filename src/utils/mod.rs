
use std::{error::Error, fs::File, io::{self, Read}};

use thiserror::Error;


#[derive(Error,Debug)]
pub enum CustomError {
    #[error("{0}")]
    Message(String)
}


pub struct Input {
    pub query: String,
    pub path: String,
    pub ignore_case: bool
}

impl Input {
    pub fn build(mut args: impl Iterator<Item=String>,ignore_case: bool) -> Result<Self,CustomError> {
        args.next();

        let args1 = match args.next() {
            Some(query) => query,
            None => return Err(CustomError::Message("query not found".to_string()))
        };

        let args2 = match args.next() {
            Some(file_path) => file_path,
            None => return Err(CustomError::Message("file path not found".to_string()))      

        };

        Ok(Self {
            query: args1,
            path: args2,
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
