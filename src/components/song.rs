use leptos::*;

use crate::{components::albumart::AlbumArt, models::song::Song};

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<(), ServerFnError> {
    logging::log!("Update song played");
    match Song::set_played(song_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[component]
pub fn SongListView() -> impl IntoView {
    let song_getter = use_context::<Resource<(), Result<Vec<Song>, ServerFnError>>>()
        .expect("Expected to find the songs getter provided");

    view! {
      <Suspense fallback=move || {
          view! { <div></div> }
      }>
        {move || match song_getter.get() {
            Some(result) => {
                match result {
                    Ok(songs) => {
                        view! {
                          <div>
                            <table class="table">
                              <thead></thead>
                              <tbody>
                                <For
                                  each=move || songs.clone().into_iter().enumerate()
                                  key=|(i, _)| *i
                                  children=move |(_, song)| {
                                      view! { <SongView song/> }
                                  }
                                />

                              </tbody>
                            </table>
                          </div>
                        }
                    }
                    Err(_) => view! { <div>"Something went wrong"</div> },
                }
            }
            None => view! { <div>"No setlist"</div> },
        }}

      </Suspense>
    }
}

#[component]
pub fn SongView(song: Song) -> impl IntoView {
    let song_filepath = song.audio_file_path.clone();

    let song_getter = use_context::<Resource<(), Result<Vec<Song>, ServerFnError>>>()
        .expect("Expected to find the songs getter provided");
    let set_song_id = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a set_played signal provided");

    let set_played_action = create_action(
        |input: &(i32, Resource<(), Result<Vec<Song>, ServerFnError>>)| {
            let input = input.to_owned();
            async move {
                set_song_played(input.0).await.map(|res| {
                    input.1.refetch();
                    res
                })
            }
        },
    );

    view! {
      <tr>
        <td>
          <div class="flex items-center gap-3">
            <div class="avatar">
              <div class="mask mask-squircle w-12 h-12">
                <AlbumArt song_filepath/>
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
          <button
            type="button"
            class="btn btn-outline btn-error"
            on:click=move |_| {
                set_played_action.dispatch((song.id, song_getter));
            }
          >

            <i class="fa-solid fa-check"></i>
          </button>
        </td>
      </tr>
    }
}
