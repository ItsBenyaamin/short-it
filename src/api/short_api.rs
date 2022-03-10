pub mod short_api {
    use warp::Filter;
    use crate::api::{add_short, get_all, login_base};
    use crate::ShortItClient;

    pub async fn setup_endpoints(client: ShortItClient) {
        let accept = warp::header::exact("Accept", "application/json");
        let content_type = warp::header::exact("Content-Type", "application/json");

        let login = warp::path!("login")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(login_base);


        let list_all = warp::path!("all")
            .and(warp::get())
            .and(content_type)
            .and(with_client(client.clone()))
            .and_then(get_all);

        let add_short = warp::path!("add")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(add_short);

        let routes = login
            .or(list_all)
            .or(add_short);

        warp::serve(routes)
            .run(([127, 0, 0, 1], 4500))
            .await;
    }

    fn with_client(client: ShortItClient) -> impl Filter<Extract = (ShortItClient,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || client.clone())
    }

}