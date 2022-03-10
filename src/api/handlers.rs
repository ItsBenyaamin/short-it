use bcrypt::verify;
use warp::hyper::{Body, StatusCode};
use warp::http::Response;
use warp::{head, Reply};
use crate::api::*;
use crate::data::Short;
use crate::ShortItClient;

pub async fn login_base(body: LoginRequest, short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    if body.username == client.config.username {
        match verify(body.password, client.config.password.as_str()) {
            Ok(verify_result) => {
                if verify_result {
                    let result = client.login();
                    return Ok(warp::reply::Response::new(result.into()))
                }
            }
            Err(_) => {}
        }
    }
    Ok(warp::reply::with_status("wrong credential!", StatusCode::UNAUTHORIZED).into_response())
}

pub async fn get_all(short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    let mut response = warp::reply::Response::new(client.list_of_shorts().into());
    let json_header_value = warp::http::HeaderValue::from_str("application/json").unwrap();
    response.headers_mut().append("Content-Type", json_header_value);
    Ok(response)
}