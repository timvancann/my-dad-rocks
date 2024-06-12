use leptos::*;

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

#[component]
pub fn SongItem(song: Song) -> impl IntoView {
    view! {
      <div class="mr-4">
      <div class="flex items-center gap-1">
        <div class="w-12 h-12">
          <AlbumArt mid=song.release_mid.unwrap() width=48 height=48/>
        </div>
        <div class="grow">
          <div class="font-bold text-sm text-left">{song.title}</div>
          <div class="text-xs opacity-70 text-left">{song.artist}</div>
        </div>
      <div class="text-xs opacity-80 text-right mx-4">{song.bpm} bpm</div>
      </div>
      </div>
    }
}
