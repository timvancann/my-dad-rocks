use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub last_played_at: Option<NaiveDate>,
    pub audio_file_path: String,
    pub album_art: String,
    pub should_play: bool,
    pub lyrics: String,
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
            album_art: Song::get_picture_as_base64(row.audio_file_path),
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
            audio_file_path: row.audio_file_path.clone(),
            album_art: Song::get_picture_as_base64(row.audio_file_path),
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
            last_played_at: row.last_played_at,
            audio_file_path: row.audio_file_path.clone(),
            album_art: Song::get_picture_as_base64(row.audio_file_path),
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
    fn get_picture_as_base64(audio_path: String) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        use id3::{Tag, TagLike};
        let file_path = format!("./assets/{}", audio_path);

        if let Ok(tag) = Tag::read_from_path(file_path) {
            if let Some(pic) = tag.pictures().next() {
                return STANDARD.encode(&pic.data);
            }
        };

        return "".to_string();
    }

    #[cfg(feature = "ssr")]
    pub async fn update_lyrics(id: i32, lyrics: String) -> Result<()> {
        sqlx::query!("UPDATE songs SET lyrics = $1 WHERE id = $2", lyrics, id)
            .execute(crate::database::get_db())
            .await
            .map(|_| ())
    }
}
