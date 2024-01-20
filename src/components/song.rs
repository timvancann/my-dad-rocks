use leptos::*;

use crate::{components::albumart::AlbumArt, error_template::ErrorTemplate, models::song::Song};

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
          <table class="table">
            <thead></thead>
            <tbody>
              {songs
                  .clone()
                  .into_iter()
                  .map(move |song| view! { <SongView song pick_song set_song_played/> })
                  .collect_view()}
            </tbody>
          </table>
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
      <tr>
        <td>
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
        </td>
        <td>
          <button
            type="button"
            class="btn btn-primary btn-circle"
            on:click=move |_| {
                set_song_id.update(|id| *id = Some(song.id));
            }
          >

            <i class="fa fa-play"></i>
          </button>
        </td>
        <td>
          <div class="join">
            <button
              type="button"
              class="btn btn-outline btn-error join-item"
              on:click=move |_| { set_song_played.dispatch(SetSongPlayed { song_id: song.id }) }
            >

              <i class="fa-solid fa-check"></i>
            </button>
            {if song.is_practice {
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
        </td>
      </tr>
    }
}
