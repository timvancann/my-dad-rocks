use leptos::*;

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

#[component]
pub fn SongItem(song: Song) -> impl IntoView {
    view! {
      <div class="flex items-center gap-1">
        <div class="w-12 h-12">
          <AlbumArt title=song.sanitized_title.to_string() width=48 height=48/>
        </div>
        <div>
          <div class="font-bold text-sm text-left">{song.title}</div>
          <div class="text-xs opacity-70 text-left">{song.artist}</div>
        </div>
      </div>
    }
}
