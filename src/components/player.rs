use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub song: Song,
    pub all_songs: Vec<Song>,
}

impl PlayerData {
    pub fn next_song(&self) -> Song {
        self.all_songs
            .iter()
            .cycle()
            .skip_while(|&s| s.id != self.song.id)
            .nth(1)
            .unwrap()
            .clone()
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let get_playerdata =
        use_context::<ReadSignal<Option<PlayerData>>>().expect("get_song_id context expected");

    view! {
      <div class="flex flex-col items-center justify-center sticky top-0 z-10 mx-2">
        <Transition fallback=|| {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"Loading song"</div> }
        }>
          {move || {
              match get_playerdata.get() {
                  Some(player_data) => {
                      view! {
                        <div class="flex-1 flex-col w-full rounded-md shadow-lg pb-2 bg-ctp-surface1 mt-1 p-1">
                          <SelectedSongView song=player_data.song.clone()/>
                          <AudioPlayer player_data/>
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
fn AudioPlayer(player_data: PlayerData) -> impl IntoView {
    let set_player_data =
        use_context::<WriteSignal<Option<PlayerData>>>().expect("set_song_id context expected");
    let url = player_data.song.gs_url.clone();

    view! {
      <div class="flex mt-1">
        <audio class="grow" controls autoplay preload="metadata"
        on:ended=move |_| {
            let new_data = Some(PlayerData {
                      song: player_data.next_song(),
                      all_songs: player_data.all_songs.clone(),
                  });
            set_player_data
              .update(|data| {
                  *data = new_data
              })
        }>
        <source src=url/>
        </audio>
      </div>
    }
}
