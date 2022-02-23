use serde::{Serialize, Deserialize};

pub trait ResponseData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: ResponseData> {
    pub error: String,
    pub status_code: u16,
    pub data: T
}

impl <T: ResponseData> Response<T> {

    pub fn with_data(_data: T, _status_code: u16) -> Self {
        Response {
            error: String::from(""),
            status_code: _status_code,
            data: _data
        }
    }

    pub fn with_error(_error: String, _status_code: u16, _data: T) -> Self {
        Response {
            error: _error,
            status_code: _status_code,
            data: _data
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
    pub fn from(_username: String, _token: String) -> Self {
        Self {
            username: _username,
            token: _token
        }
    }
}

impl EmptyData {
    pub fn from(_d: String) -> Self {
        Self {
            data: _d
        }
    }
}

impl ResponseData for LoginData  {}
impl ResponseData for String  {}