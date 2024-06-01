#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use my_dad_rocks::app::*;
    use my_dad_rocks::database::init_db;
    use my_dad_rocks::fileserv::file_and_error_handler;
    use tower_http::cors::{Any, CorsLayer};

    let _ = init_db().await;
    //let _ = init_thumbnails();

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let cors = CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(cors)
        .with_state(leptos_options);

    log::info!("listening on https://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
