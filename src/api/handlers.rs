use std::net::SocketAddr;
use bcrypt::verify;
use warp::hyper::{Body, StatusCode};
use warp::http::{HeaderMap, Response, Uri};
use warp::Reply;
use crate::api::*;
use crate::ShortItClient;

pub async fn base(hash: String, short_client: ShortItClient, addr: Option<SocketAddr>, headers: HeaderMap) -> Result<Response<Body>, warp::Rejection> {
    let client = short_client.lock().await;
    let client_ip =  if addr.is_some() {
        addr.unwrap().ip().to_string()
    }else {
        "".to_owned()
    };

    let referer = match headers.get("Referer") {
        None => { String::from("") }
        Some(header_value) => { header_value.to_str().unwrap().to_owned() }
    };

    return match client.get_url(hash, client_ip, referer) {
        Some(result) => {
            Ok(warp::redirect(Uri::try_from(result).unwrap()).into_response())
        }
        None => {
            Ok(warp::reply::with_status(String::from("not found!"), StatusCode::NOT_FOUND).into_response())
        }
    }
}

pub async fn login(body: LoginRequest, short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    let mut client = short_client.lock().await;
    if body.username == client.config.username {
        if let Ok(verify_result) = verify(body.password, client.config.password.as_str()) {
            if verify_result {
                let result = client.login();
                return Ok(warp::reply::Response::new(result.into()))
            }
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