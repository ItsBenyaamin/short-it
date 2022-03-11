pub mod api {
    use warp::Filter;
    use crate::api::{add_short, delete_short, edit_short, get_all, login_base};
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

        let add = warp::path!("add")
            .and(warp::post())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(add_short);

        let edit = warp::path!("edit")
            .and(warp::patch())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(edit_short);

        let delete = warp::path!("delete")
            .and(warp::delete())
            .and(accept)
            .and(content_type)
            .and(warp::body::json())
            .and(with_client(client.clone()))
            .and_then(delete_short);

        let routes = login
            .or(list_all)
            .or(add)
            .or(edit)
            .or(delete);

        warp::serve(routes)
            .run(([127, 0, 0, 1], 4500))
            .await;
    }

    fn with_client(client: ShortItClient) -> impl Filter<Extract = (ShortItClient,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || client.clone())
    }

}