use serde::{Serialize, Deserialize};

pub trait ResponseData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: ResponseData> {
    pub error: String,
    pub status_code: u16,
    pub data: T
}

impl <T: ResponseData> Response<T> {

    pub fn with_data(data: T, status_code: u16) -> Self {
        Response {
            error: String::from(""),
            status_code,
            data
        }
    }

    pub fn with_error(error: String, status_code: u16, _data: T) -> Self {
        Response {
            error,
            status_code,
            data
        }
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginData {
    pub username: String,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyData {
    pub data: String
}

impl LoginData {
    pub fn from(username: String, token: String) -> Self {
        Self {
            username,
            token
        }
    }
}

impl EmptyData {
    pub fn from(data: String) -> Self {
        Self {
            data
        }
    }
}

impl ResponseData for LoginData  {}
impl ResponseData for String  {}