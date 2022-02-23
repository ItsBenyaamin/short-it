use bcrypt::{BcryptResult, verify};
use warp::hyper::{Body, StatusCode};
use warp::http::Response;
use warp::Reply;
use crate::api::*;
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


pub async fn something(short_client: ShortItClient) -> Result<Response<Body>, warp::Rejection> {
    unimplemented!()
}