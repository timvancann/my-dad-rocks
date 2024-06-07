use clap::{Parser, Subcommand};

use my_dad_rocks::database::init_db;
use my_dad_rocks::models::song::Song;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Api,
    Generate,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Api => {
            run_api().await;
        }
        Commands::Generate => {
            generate_thumbnails().await?;
        }
    }
    Ok(())
}

async fn generate_thumbnails() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;

    let _ = init_db().await;
    let songs = Song::get_all().await?;
    for song in songs.into_iter() {
        if let Some(mid) = song.release_mid {
            print!(
                "Downloading art for {} with release id {}",
                song.title, &mid
            );
            let path = format!("public/coverart/{}.jpeg", &mid);
            if Path::new(&path).exists() {
                print!("\t [Exists]\n");
                continue;
            }

            if [
                "0d5f0dc2-b597-4b6c-9a6f-49b70b8e23b6".to_string(), // van halen
                "8d0bc6d4-8700-44e8-90c8-b86c23e7ff14".to_string(), // alive
                "e01bc4d7-6482-4f48-98fc-ae36fc8e9ff2".to_string(), // breakfast
                "b9c54760-423f-3065-802e-a14313c87934".to_string(), // johnny
                "8a98a665-8359-4deb-8f99-2391fedddcb3".to_string(), // just
                "73fa4781-d526-32e6-b23e-c8d5dc672429".to_string(), // fly
                "3dc6075a-ee1b-4d3d-bc4b-f2a8ab66c806".to_string(), // train
                "ae22ec48-abbb-4adb-8c6c-4cfc296ca3cb".to_string(), // love
                "ab9e6f50-b248-4ed2-a591-1f175e609e44".to_string(), // knows
                "80e9733e-51fa-434b-8131-9f1710aae2d1".to_string(), // smoor
                "cfbc501c-6f41-4618-8bb1-90bb8ebd936f".to_string(), // stuck
                "a0a2b395-7989-4ec7-99f9-9bc9425c53b7".to_string(), // time
            ]
            .contains(&mid)
            {
                print!("\t [Skip]\n");
                continue;
            }

            match download_art(mid, path).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error downloading art for {}: {:?}", song.title, e);
                }
            }
        }
    }
    Ok(())
}

async fn download_art(release_id: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    use musicbrainz_rs::entity::CoverartResponse;
    use musicbrainz_rs::prelude::*;

    let coverart_response = musicbrainz_rs::entity::release::Release::fetch_coverart()
        .id(&release_id)
        .execute()
        .await?;

    let url: Option<String> = match coverart_response {
        CoverartResponse::Json(coverart) => coverart.images[0].thumbnails.res_250.clone(),
        CoverartResponse::Url(url) => Some(url),
    };

    if let Some(url) = url {
        let resp = reqwest::get(url).await?;
        let image = image::load_from_memory(&resp.bytes().await?)?;
        image.save(path)?;
    } else {
        println!("No cover art found for {}", release_id);
    }
    Ok(())
}

#[cfg(feature = "ssr")]
async fn run_api() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use my_dad_rocks::app::*;
    use my_dad_rocks::database::init_db;
    use my_dad_rocks::fileserv::file_and_error_handler;
    use tower_http::cors::{Any, CorsLayer};

    let _ = init_db().await;

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
