pub mod short_api {
    use warp::Filter;
    use crate::api::{login_base, something};
    use crate::ShortItClient;

    pub async fn setup_endpoints(client: ShortItClient) {
        let accept = warp::header::exact("Accept", "application/json");
        let content_type = warp::header::exact("Content-Type", "application/json");
        let login = warp::path!("login")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client))
            .and_then(login_base);



        warp::serve(login)
            .run(([127, 0, 0, 1], 4500))
            .await;
    }

    fn with_client(client: ShortItClient) -> impl Filter<Extract = (ShortItClient,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || client.clone())
    }

}