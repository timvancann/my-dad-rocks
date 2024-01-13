use core::panic;

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
