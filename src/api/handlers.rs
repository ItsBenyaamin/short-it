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

pub async fn add_short(body: AddRequest, short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    if body.token != client.config.token {
        return Ok(warp::reply::with_status(String::from("error"),
                                           StatusCode::UNAUTHORIZED).into_response())
    }
    let result = client.short_with(body.url, body.until);
    Ok(warp::reply::Response::new(result.into()))
}

pub async fn edit_short(body: EditRequest, short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    if body.token != client.config.token {
        return Ok(warp::reply::with_status(String::from("error"),
                                           StatusCode::UNAUTHORIZED).into_response())
    }
    let result = client.edit_short(body.hash, body.url, body.until);
    Ok(warp::reply::Response::new(result.into()))
}

pub async fn delete_short(body: RemoveRequest, short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    if body.token != client.config.token {
        return Ok(warp::reply::with_status(String::from("error"),
                                           StatusCode::UNAUTHORIZED).into_response())
    }
    let result = client.delete_short(body.hash);
    Ok(warp::reply::Response::new(result.into()))
}