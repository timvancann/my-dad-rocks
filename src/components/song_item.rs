use leptos::*;

use crate::{components::albumart::AlbumArt, models::song::Song};

#[component]
pub fn SongItem(song: Song) -> impl IntoView {
    view! {
      <div class="flex items-center gap-3">
        <div class="avatar">
          <div class="mask mask-squircle w-12 h-12">
            <AlbumArt base64_encoded_string=song.album_art/>
          </div>
        </div>
        <div>
          <div class="font-bold">{song.title}</div>
          <div class="text-sm opacity-70">{song.artist}</div>
          <div class="badge badge-outline text-sm opacity-50">
            {match song.last_played_at {
                Some(d) => d.format("%d-%m-%Y").to_string(),
                None => "Nooit".to_string(),
            }}

          </div>
        </div>
      </div>
    }
}
