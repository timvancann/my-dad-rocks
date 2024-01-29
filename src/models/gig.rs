use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::song::Song;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SongKind {
    Break(i32),
    Song(Song),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct GigModel {
    pub id: i32,
    pub venue: String,
    pub date: NaiveDate,
    pub songs: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Gig {
    pub id: i32,
    pub venue: String,
    pub date: NaiveDate,
    pub songs: Vec<SongKind>,
    pub unselected_songs: Vec<Song>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MoveKind {
    Up,
    Down,
}

impl Gig {
    #[cfg(feature = "ssr")]
    pub async fn get_by_id(id: i32) -> Result<Self, sqlx::Error> {
        let songs = Song::get_all().await?;
        let gig = sqlx::query_as!(GigModel, "SELECT * FROM gigs WHERE id = $1", id)
            .fetch_one(crate::database::get_db())
            .await?;
        Ok(Gig {
            id: gig.id,
            venue: gig.venue,
            date: gig.date,
            songs: gig
                .songs
                .iter()
                .map(|s| {
                    if s < &0 {
                        SongKind::Break(*s)
                    } else {
                        SongKind::Song(songs.iter().find(|song| song.id == *s).unwrap().clone())
                    }
                })
                .collect(),
            unselected_songs: songs
                .into_iter()
                .filter(|s| !gig.songs.contains(&s.id))
                .collect(),
        })
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
}
