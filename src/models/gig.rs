use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::song::Song;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum SongKind {
    Break(i32),
    Song(Song),
}

impl Default for SongKind {
    fn default() -> Self {
        Self::Break(-1)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GigModel {
    pub id: i32,
    pub venue: String,
    pub date: NaiveDate,
    pub time: Option<String>,
    pub songs: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Gig {
    pub id: i32,
    pub venue: String,
    pub date: NaiveDate,
    pub time: Option<String>,
    pub songs: Vec<GigSong>,
    pub unselected_songs: Vec<Song>,
}

impl Default for Gig {
    fn default() -> Self {
        Self {
            id: 0,
            venue: "".to_string(),
            date: NaiveDate::default(),
            time: None,
            songs: Vec::default(),
            unselected_songs: Vec::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Copy)]
pub enum MoveKind {
    Up,
    Down,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GigSong {
    pub index: usize,
    pub id: i32,
    pub song: Option<Song>,
}

impl Gig {
    #[cfg(feature = "ssr")]
    pub async fn get_by_id(id: i32) -> Result<Self, sqlx::Error> {
        let all_songs = Song::get_all().await?;
        let gig = sqlx::query_as!(GigModel, "SELECT * FROM gigs WHERE id = $1", id)
            .fetch_one(crate::database::get_db())
            .await?;

        let mut songs_indexed: Vec<GigSong> = Vec::default();
        let mut break_count = 0usize;
        for (index, song_id) in gig.songs.iter().enumerate() {
            if song_id < &0 {
                break_count += 1;
                songs_indexed.push(GigSong {
                    index: index - break_count,
                    id: *song_id,
                    song: None,
                });
            } else {
                let song = all_songs
                    .iter()
                    .find(|song| song.id == *song_id)
                    .unwrap()
                    .clone();
                songs_indexed.push(GigSong {
                    index: index - break_count,
                    id: *song_id,
                    song: Some(song),
                });
            }
        }

        let gig = Gig {
            id: gig.id,
            venue: gig.venue,
            time: gig.time,
            date: gig.date,
            songs: songs_indexed,
            unselected_songs: all_songs
                .into_iter()
                .filter(|s| !gig.songs.contains(&s.id))
                .collect(),
        };

        Ok(gig)
    }

    #[cfg(feature = "ssr")]
    pub async fn add_song(gig_id: i32, song_id: i32) -> Result<(), sqlx::Error> {
        use std::cmp::min;

        let song_id = if song_id < 0 {
            min(
                sqlx::query!("SELECT songs FROM gigs WHERE id = $1", gig_id)
                    .map(|row| row.songs.into_iter().min())
                    .fetch_one(crate::database::get_db())
                    .await?
                    .unwrap(),
                0,
            ) - 1
        } else {
            song_id
        };

        sqlx::query!(
            "UPDATE gigs SET songs = array_append(songs, $1) WHERE id = $2",
            song_id,
            gig_id,
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn remove_song(gig_id: i32, song_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE gigs SET songs = array_remove(songs, $1) WHERE id = $2",
            song_id,
            gig_id,
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn move_song(
        gig_id: i32,
        song_id: i32,
        move_kind: MoveKind,
    ) -> Result<(), sqlx::Error> {
        let mut songs = sqlx::query!("SELECT * from gigs WHERE id = $1", gig_id)
            .map(|row| row.songs)
            .fetch_one(crate::database::get_db())
            .await?;

        let index_of_song = songs.iter().position(|s| *s == song_id).unwrap();

        let new_index = match move_kind {
            MoveKind::Up => index_of_song - 1,
            MoveKind::Down => index_of_song + 1,
        };

        if new_index == index_of_song || new_index >= songs.len() {
            return Ok(());
        }

        songs.swap(index_of_song, new_index);

        sqlx::query!("UPDATE gigs SET songs = $1 WHERE id = $2", &songs, gig_id)
            .execute(crate::database::get_db())
            .await?;

        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn set_venue(gig_id: i32, venue: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE gigs SET venue = $1 WHERE id = $2",
            venue,
            gig_id as i32
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }
    #[cfg(feature = "ssr")]
    pub async fn set_time(gig_id: i32, time: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE gigs SET time = $1 WHERE id = $2",
            time,
            gig_id as i32
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }
    #[cfg(feature = "ssr")]
    pub async fn set_date(gig_id: i32, date: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE gigs SET date = $1 WHERE id = $2",
            match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => return Err(sqlx::Error::RowNotFound),
            },
            gig_id as i32
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<GigModel>, sqlx::Error> {
        sqlx::query_as!(GigModel, "SELECT * FROM gigs ORDER BY date ASC")
            .fetch_all(crate::database::get_db())
            .await
    }
    #[cfg(feature = "ssr")]
    pub async fn create() -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
INSERT INTO gigs (venue, date, songs)  VALUES ('Nieuw', $1, '{}');
",
            NaiveDate::default()
        )
        .execute(crate::database::get_db())
        .await?;
        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn remove(gig_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM gigs WHERE id = $1", gig_id)
            .execute(crate::database::get_db())
            .await?;
        Ok(())
    }
}
