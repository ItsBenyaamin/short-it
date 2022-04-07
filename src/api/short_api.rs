pub mod api {
    use warp::Filter;
    use crate::api::{base, login, add_short, delete_short, edit_short, get_all};
    use crate::ShortItClient;

    pub async fn setup_endpoints(client: ShortItClient) {
        let accept = warp::header::exact("Accept", "application/json");
        let content_type = warp::header::exact("Content-Type", "application/json");

        let base_path = warp::path!("r" / String)
            .and(with_client(client.clone()))
            .and(warp::addr::remote())
            .and(warp::header::headers_cloned())
            .and_then(base);

        let login_path = warp::path!("api" / "login")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(login);

        let list_all_path = warp::path!("api" / "all")
            .and(warp::get())
            .and(content_type)
            .and(warp::header::optional("api-key"))
            .and(with_client(client.clone()))
            .and_then(get_all);

        let add_path = warp::path!("api" / "add")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(warp::header::optional("api-key"))
            .and(with_client(client.clone()))
            .and_then(add_short);

        let edit_path = warp::path!("api" / "edit")
            .and(warp::patch())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(warp::header::optional("api-key"))
            .and(with_client(client.clone()))
            .and_then(edit_short);

        let delete_path = warp::path!("api" / "delete")
            .and(warp::delete())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(warp::header::optional("api-key"))
            .and(with_client(client.clone()))
            .and_then(delete_short);

        let routes = login_path
            .or(list_all_path)
            .or(add_path)
            .or(edit_path)
            .or(delete_path)
            .or(base_path);

        //TODO get port from config file
        warp::serve(routes)
            .run(([127, 0, 0, 1], 4500)).await;
    }

    fn with_client(client: ShortItClient) -> impl Filter<Extract = (ShortItClient,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || client.clone())
    }

}