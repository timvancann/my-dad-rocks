use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub last_played_at: Option<NaiveDate>,
    pub audio_file_path: String,
    pub gs_url: Option<String>,
    pub album_art: String,
    pub should_play: bool,
    pub lyrics: String,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: 0,
            artist: "".to_string(),
            title: "".to_string(),
            last_played_at: None,
            gs_url: None,
            audio_file_path: "".to_string(),
            album_art: "".to_string(),
            should_play: false,
            lyrics: "".to_string(),
        }
    }
}

#[cfg(feature = "ssr")]
type Result<T> = std::result::Result<T, sqlx::Error>;

impl Song {
    #[cfg(feature = "ssr")]
    pub async fn get(song_id: i32) -> Result<Self> {
        use super::setlist::Setlist;

        let song_in_setlist = Setlist::song_in_setlist(song_id).await?;

        sqlx::query!(
            "
        SELECT 
          * 
        FROM songs as s 
        WHERE s.id = $1",
            song_id
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            last_played_at: row.last_played_at,
            audio_file_path: row.audio_file_path.clone(),
            gs_url: row.gs_url,
            album_art: Song::get_picture_as_base64(row.audio_file_path, ThumbnailType::Thumbnail),
            should_play: song_in_setlist,
            lyrics: row.lyrics,
        })
        .fetch_one(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all_in_setlist(setlist_id: i32) -> Result<Vec<Song>> {
        use super::setlist::Setlist;
        let setlist_songs = Setlist::get_by_id(setlist_id).await?.songs;
        sqlx::query!(
            "SELECT 
              * 
            FROM songs"
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            last_played_at: row.last_played_at,
            gs_url: row.gs_url,
            audio_file_path: row.audio_file_path.clone(),
            album_art: Song::get_picture_as_base64(row.audio_file_path, ThumbnailType::Thumbnail),
            should_play: setlist_songs.contains(&row.id),
            lyrics: row.lyrics,
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<Self>> {
        use super::setlist::Setlist;

        let setlist_songs = Setlist::get().await?.songs;
        sqlx::query!(
            "SELECT 
              * 
            FROM songs as s 
            ORDER BY 
              s.title ASC"
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            gs_url: row.gs_url,
            last_played_at: row.last_played_at,
            audio_file_path: row.audio_file_path.clone(),
            album_art: Song::get_picture_as_base64(row.audio_file_path, ThumbnailType::Thumbnail),
            should_play: setlist_songs.contains(&row.id),
            lyrics: row.lyrics,
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn set_played(song_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE songs SET last_played_at = CURRENT_DATE WHERE id = $1",
            song_id
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }

    #[cfg(feature = "ssr")]
    pub fn get_picture_as_base64(audio_path: String, thumbnail_type: ThumbnailType) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        use image::{io::Reader as ImageReader, ImageOutputFormat};
        use std::{path::PathBuf, io::Cursor};

        let dir = match thumbnail_type {
            ThumbnailType::Thumbnail => "thumbnails",
            ThumbnailType::Player => "player",
        };

        let file_path = PathBuf::from(format!("./assets/{}/{}", dir, audio_path));
        let img_path = file_path.with_extension("png");

        println!("reading image from {:?}", img_path);

        let img = ImageReader::open(img_path).unwrap().decode().unwrap();
        let mut image_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
            .unwrap();
        STANDARD.encode(image_data)
    }

    #[cfg(feature = "ssr")]
    pub async fn update_lyrics(id: i32, lyrics: String) -> Result<()> {
        sqlx::query!("UPDATE songs SET lyrics = $1 WHERE id = $2", lyrics, id)
            .execute(crate::database::get_db())
            .await
            .map(|_| ())
    }
}

pub enum ThumbnailType {
    Thumbnail,
    Player,
}
