use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::models::song::Song;

pub async fn serve_mp3(Path(id): Path<i32>) -> impl IntoResponse {
    let song = Song::get(id).await.unwrap();
    let path = PathBuf::from("assets/mp3").join(song.audio_file_path);
    let file = File::open(path).await.unwrap();

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::from_stream(stream);

    Response::builder()
        .header("Content-Type", "audio/mpeg")
        .body(body)
        .unwrap()
}
