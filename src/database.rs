use core::panic;
use std::{error::Error, fs};

static DB: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let mut retries = 0;
    while retries < 5 {
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(database_url.as_str())
            .await
        {
            Ok(pool) => {
                sqlx::migrate!()
                    .run(&pool)
                    .await
                    .expect("migrations failed");
                return pool;
            }
            Err(e) => {
                println!(
                    "Could not connect to database {}, retrying {} times, error: {}",
                    database_url, retries, e
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                retries += 1;
            }
        }
    }
    panic!("Could not connect to database");
}

pub async fn init_db() -> Result<(), sqlx::Pool<sqlx::Postgres>> {
    DB.set(create_pool().await)
}

pub fn get_db<'a>() -> &'a sqlx::PgPool {
    DB.get().expect("database unitialized")
}

pub fn init_thumbnails() -> Result<(), Box<dyn Error>> {
    use id3::Tag;
    use image::imageops::{resize, FilterType};
    use image::io::Reader as ImageReader;
    use std::io::Cursor;
    use std::path::PathBuf;

    let path = PathBuf::from("assets/mp3");
    let thumbnail_path = PathBuf::from("assets/thumbnails");
    let player_path = PathBuf::from("assets/player");
    println!("Path: {:?}", path);

    match thumbnail_path.is_dir() {
        true => {},
        false => {
            println!("Creating thumbnail path");
            fs::create_dir(&thumbnail_path)?;
        },
    };
    match player_path.is_dir() {
        true => {},
        false => {
            println!("Creating player path");
            fs::create_dir(&player_path)?;
        },
    };

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        let mut img_thumbnail = thumbnail_path.to_owned();
        let mut img_player = player_path.to_owned();

        let filename = path.with_extension("png");
        let filename = filename.file_name().unwrap();

        img_thumbnail.push(filename);
        img_player.push(filename);

        if let Ok(tag) = Tag::read_from_path(path) {
            if let Some(pic) = tag.pictures().next() {
                let img = ImageReader::new(Cursor::new(&pic.data))
                    .with_guessed_format()
                    .expect("Could not guess format")
                    .decode()
                    .expect("Could not decode image");

                let resized = resize(&img, 48, 48, FilterType::CatmullRom);
                let _ = resized.save(&img_thumbnail);
                println!("Resized image with name: {:?}", img_thumbnail);

                let resized = resize(&img, 80, 80, FilterType::CatmullRom);
                let _ = resized.save(&img_player);
                println!("Resized image with name: {:?}", img_player);
            }
        };
    }
    Ok(())
}
