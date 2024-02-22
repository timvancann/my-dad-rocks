use leptos::*;

use crate::{components::albumart::AlbumArt, models::song::Song};

#[component]
pub fn SongItem(song: Song) -> impl IntoView {
    view! {
      <a href=format!("/lyric/{}", song.id)>
        <div class="flex items-center place-items-center gap-1">
          <div class="">
            <div class="w-12 h-12">
              <AlbumArt base64_encoded_string=song.album_art/>
            </div>
          </div>
          <div>
            <div class="font-bold text-sm">{song.title}</div>
            <div class="text-xs opacity-70">{song.artist}</div>
            <div class="text-xs opacity-50">
              {match song.last_played_at {
                  Some(d) => d.format("%d-%m-%Y").to_string(),
                  None => "Nooit".to_string(),
              }}

            </div>
          </div>
        </div>
      </a>
    }
}
