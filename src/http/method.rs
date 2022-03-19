use std::{str::FromStr};

#[derive(Debug)]
pub enum Method {
    DELETE,
    GET,
    HEAD,
    POST,
    PUT,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(String::from("Error"))
        }
    }
}