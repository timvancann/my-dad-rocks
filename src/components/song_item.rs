use leptos::*;

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

#[component]
pub fn SongItem(song: Song) -> impl IntoView {
    view! {
      <a href=format!("/lyric/{}", song.id)>
        <div class="flex items-center place-items-center gap-1">
          <div class="w-12 h-12">
             <AlbumArt title={song.sanitized_title.to_string()} width=48 height=48/>
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
