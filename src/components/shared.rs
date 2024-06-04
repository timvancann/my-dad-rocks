use leptos::*;

use crate::components::player::PlayerData;

#[component]
pub fn Horizontal() -> impl IntoView {
    view! { <div class="h-px m-3 mt-6 bg-ctp-surface1 border-0"></div> }.into_view()
}

#[component]
pub fn AlbumArt(title: String, width: u32, height: u32) -> impl IntoView {
    let mut art = title;
    art.push_str(".png");
    let path = format!("/album_art/{}", art);

    view! { <img src=path width=width height=height/> }
}

#[component]
pub fn PlayButton(song_id: i32, setlist_id: i32) -> impl IntoView {
    let set_player_data = use_context::<WriteSignal<Option<PlayerData>>>()
        .expect("Expected to have a set_played signal provided");

    view! {
      <button
        type="button"
        class="border-0 rounded-lg py-3 w-16 shadow-lg bg-ctp-green text-ctp-mantle"
        on:click=move |_| {
            set_player_data
                .update(|data| {
                    *data = Some(PlayerData { song_id, setlist_id });
                });
        }
      >

        <i class="fa fa-play"></i>
      </button>
    }
}

#[component]
pub fn LyricsButton(song_id: i32) -> impl IntoView {
    view! {
      <a href=format!("/lyric/{}", song_id)>
        <button
          type="button"
          class="border-0 rounded-md px-3 py-2 shadow-lg bg-ctp-lavender text-ctp-mantle"
        >
          <i class="fa-solid fa-align-left"></i>
          Lyrics
        </button>
      </a>
    }
}
