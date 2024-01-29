use leptos::*;

use crate::{components::song_item::SongItem, models::song::Song};

use super::all_songs::{HandPickSong, SetSongPlayed};

#[component]
pub fn SongListView(
    songs: Vec<Song>,
    pick_song: Action<HandPickSong, Result<(), ServerFnError>>,
    set_song_played: Action<SetSongPlayed, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
      <Suspense fallback=move || view! { <div></div> }>
        <div>
          <div class="grid grid-cols-8 gap-2">
              {songs
                  .clone()
                  .into_iter()
                  .map(move |song| view! { <SongView song pick_song set_song_played/> })
                  .collect_view()}
          </div>
        </div>
      </Suspense>
    }
}

#[component]
pub fn SongView(
    song: Song,
    pick_song: Action<HandPickSong, Result<(), ServerFnError>>,
    set_song_played: Action<SetSongPlayed, Result<(), ServerFnError>>,
) -> impl IntoView {
    let set_song_id = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a set_played signal provided");

    view! {
        <div class="col-span-5 ml-4">
          <SongItem song=song.clone()/>
        </div>
        <div class="col-span-1">
          <button
            type="button"
            class="btn btn-primary btn-circle"
            on:click=move |_| {
                set_song_id.update(|id| *id = Some(song.id));
            }
          >

            <i class="fa fa-play"></i>
          </button>
        </div>
        <div class="col-span-2">
          <div class="join">
            <button
              type="button"
              class="btn btn-outline btn-error join-item"
              on:click=move |_| { set_song_played.dispatch(SetSongPlayed { song_id: song.id }) }
            >

              <i class="fa-solid fa-calendar-day"></i>
            </button>
            {if song.should_play {
                view! {
                  <button
                    type="button"
                    class="btn btn-outline btn-error btn-disabled join-item"
                    on:click=move |_| { pick_song.dispatch(HandPickSong { song_id: song.id }) }
                  >

                    <i class="fa-solid fa-bookmark"></i>
                  </button>
                }
            } else {
                view! {
                  <button
                    type="button"
                    class="btn btn-outline btn-error join-item"
                    on:click=move |_| { pick_song.dispatch(HandPickSong { song_id: song.id }) }
                  >

                    <i class="fa-solid fa-bookmark"></i>
                  </button>
                }
            }}

          </div>
        </div>
    }
}
