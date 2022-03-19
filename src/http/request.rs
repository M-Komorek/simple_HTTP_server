use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str::{self, Utf8Error};

use super::Method;
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path_and_query, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method = method.parse()?;
        let mut path = "";
        let mut query_string = None;

        if let Some(i) =  path_and_query.find('?') {
            query_string = Some(QueryString::from(&path_and_query[i+1..]));
            path = &path_and_query[..i];
        } 

        Ok(Self{ path, query_string, method})
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, mark) in request.chars().enumerate() {
        if mark == ' ' || mark == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    return None;
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<String> for ParseError {
    fn from(_: String) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::query_string::Value;

    impl Method {
        fn to_u8(&self) -> u8 {
            match self {
                Method::DELETE => 0,
                Method::GET => 1,
                Method::HEAD => 2,
                Method::POST => 3,
                Method::PUT => 4,
            }
        }
    }

    #[test]
    fn request_should_be_created_from_buffer() {
        let buffer = b"GET /home?name=Batman HTTP/1.1\r";
        let request = Request::try_from(&buffer[..]).unwrap();

        assert_eq!(request.path(), "/home");
        assert_eq!(Method::GET as u8, request.method().to_u8());
        
        let option_name = request.query_string().unwrap().data.get("name");
        if let Some(name) = option_name {
            if let Value::Single(str) = name {
                assert_eq!(str, &"Batman");
            } else {
                assert!(false, "Wrong Value type!")
            }
        } else {
            assert!(false, "No key in map!")
        }
    }

    fn check_for_error_raise(buffer: &[u8], error_message: &str) {
        if let Err(parse_error) = Request::try_from(&buffer[..]) {
            assert_eq!(parse_error.message(), error_message);
        } else {
            assert!(false, "Error \'{}\' was not raised!", error_message);
        }
    }

    #[test]
    fn request_should_raise_error_when_parse_of_method_failed() {
        let buffer = b"WRONG";
        check_for_error_raise(buffer, "Invalid Request");
    }

    #[test]
    fn request_should_raise_error_when_parse_of_path_failed() {
        let buffer = b"GET / ";
        check_for_error_raise(buffer, "Invalid Request");
    }

    #[test]
    fn request_should_raise_error_when_received_unsupported_protocol() {
        let buffer = b"POST ?name=Batman HTTP/3.1\r";
        check_for_error_raise(buffer, "Invalid Protocol");
    }

    #[test]
    fn request_should_raise_error_when_parse_method_failed() {
        let buffer = b"WRONG ?name=Batman HTTP/1.1\r";
        check_for_error_raise(buffer, "Invalid Method");
    }
}