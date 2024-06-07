use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub song_id: i32,
    pub setlist_id: i32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct PlayData {
    pub song: Song,
    pub next_song_id: i32,
    pub setlist_id: i32,
}

#[server(GetSong)]
pub async fn get_song(player_data: Option<PlayerData>) -> Result<Option<PlayData>, ServerFnError> {
    match player_data {
        Some(data) => {
            let song = Song::get(data.song_id).await?;
            let all_songs_in_setlist = Song::get_all_in_setlist(data.setlist_id).await?;
            // find next song in setlist
            let next_song = all_songs_in_setlist
                .iter()
                .cycle()
                .skip_while(|&s| s.id != data.song_id)
                .skip(1)
                .next()
                .unwrap();
            Ok(Some(PlayData {
                song,
                next_song_id: next_song.id,
                setlist_id: data.setlist_id,
            }))
        }
        _ => Ok(None),
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let get_playerdata =
        use_context::<ReadSignal<Option<PlayerData>>>().expect("get_song_id context expected");

    let player_resource = create_resource(move || get_playerdata.get(), get_song);

    view! {
      <div class="flex flex-col items-center justify-center sticky top-0 z-10 mx-2">
        <Transition fallback=|| {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"Loading song"</div> }
        }>
          {move || {
              match player_resource.get() {
                  Some(Ok(Some(play_data))) => {
                      view! {
                        <div class="flex-1 flex-col w-full rounded-md shadow-lg pb-2 bg-ctp-surface1 mt-1 p-1">
                          <SelectedSongView song=play_data.song.clone()/>
                          <AudioPlayer play_data/>
                        </div>
                      }
                          .into_view()
                  }
                  _ => view! {}.into_view(),
              }
          }}

        </Transition>
      </div>
    }
}

#[component]
fn SelectedSongView(song: Song) -> impl IntoView {
    view! {
      <div class="flex grow">
        <figure class="flex w-20 h-20 mr-4">
          <AlbumArt mid=song.release_mid.unwrap() width=80 height=80/>
        </figure>
        <div class="flex-1 flex-cols">
          <div class="text-lg font-bold">{song.title}</div>
          <div>{song.artist}</div>
        </div>
      </div>
    }
}

#[component]
fn AudioPlayer(play_data: PlayData) -> impl IntoView {
    let set_player_data =
        use_context::<WriteSignal<Option<PlayerData>>>().expect("set_song_id context expected");

    view! {
      <div class="flex mt-1">
        <audio class="grow" controls autoplay>
          // on:ended=move |_|
          // {set_player_data
          // .update(|data| {
          // log!("Setting next song id to: {:?}", play_data.next_song_id);
          // *data = Some(PlayerData {
          // song_id: play_data.next_song_id,
          // setlist_id: play_data.setlist_id,
          // });
          // })}

          <source src=play_data.song.gs_url/>
        </audio>
      </div>
    }
}
