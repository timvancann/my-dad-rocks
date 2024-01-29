use leptos::*;
use leptos_router::*;

use crate::{
    components::song_item::SongItem,
    error_template::ErrorTemplate,
    models::{
        gig::{Gig, MoveKind, SongKind},
        song::Song,
    },
};

#[server(GetGig)]
pub async fn get_gig(gig_id: Option<i32>) -> Result<Gig, ServerFnError> {
    match gig_id {
        Some(id) => Ok(Gig::get_by_id(id).await?),
        None => Err(ServerFnError::MissingArg("Missing gig_id".to_string())),
    }
}

#[server(AddSongToGig)]
pub async fn add_song_to_gig(gig_id: i32, song_id: i32) -> Result<(), ServerFnError> {
    Gig::add_song(gig_id, song_id)
        .await
        .map_err(ServerFnError::from)
}

#[server(RemoveSongFromGig)]
pub async fn remove_song_from_gig(gig_id: i32, song_id: i32) -> Result<(), ServerFnError> {
    Gig::remove_song(gig_id, song_id)
        .await
        .map_err(ServerFnError::from)
}

#[server(MoveSongInGig)]
pub async fn move_song_gig(gig_id: i32, song_id: i32, kind: MoveKind) -> Result<(), ServerFnError> {
    Gig::move_song(gig_id, song_id, kind)
        .await
        .map_err(ServerFnError::from)
}

#[derive(Params, PartialEq)]
struct GigParams {
    id: usize,
}

#[component]
pub fn Gig() -> impl IntoView {
    let edit_mode = create_rw_signal(false);

    provide_context(edit_mode);

    let params = use_params::<GigParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let remove_song = create_server_action::<RemoveSongFromGig>();
    let add_song = create_server_action::<AddSongToGig>();
    let move_song = create_server_action::<MoveSongInGig>();

    let gig_resource = create_resource(
        move || {
            (
                remove_song.version().get(),
                add_song.version().get(),
                move_song.version().get(),
                Some(id() as i32),
            )
        },
        |args| get_gig(args.3),
    );

    let unselected_songs_resource = move || match gig_resource() {
        Some(res) => match res {
            Ok(gig) => Some(Ok((gig.id, gig.unselected_songs))),
            Err(e) => Some(Err(e)),
        },
        None => None,
    };

    let edit_mode_active_class = move || match edit_mode() {
        true => "btn btn-primary btn-circle",
        false => "btn btn-primary btn-outline btn-circle",
    };

    view! {
        <div class="mb-4 mt-4">
      <div class="grid grid-cols-8 gap-2">
        <div class="col-start-7 col-span-1">
          <button
            type="button"
            class=move || edit_mode_active_class()
            on:click=move |_| { edit_mode.set(!edit_mode()) }
          >

            <i class="fa-solid fa-edit"></i>
          </button>
        </div>
            </div>

      </div>
      <Transition fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
          {move || {
              gig_resource()
                  .map(move |gig| match gig {
                      Err(e) => {
                          view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                              .into_view()
                      }
                      Ok(g) => {
                          view! {
                            <GigSongListView
                              songs=g.songs.clone()
                              gig_id=g.id
                              remove_song
                              move_song
                            />
        <div class="mb-4 mt-4">
      <div class="grid grid-cols-8 gap-2">
        <div class="col-start-7 col-span-2">
                              <button
                                type="button"
                                class="btn btn-primary btn-outline"
                                on:click=move |_| {
                                    add_song
                                        .dispatch(AddSongToGig {
                                            gig_id: g.id,
                                            song_id: -1,
                                        })
                                }
                              >

                                <i class="fa-solid fa-pause"></i>
                                Pauze
                              </button>
                            </div>
        </div>
        </div>

                          }
                              .into_view()
                      }
                  })
                  .unwrap_or_default()
          }}

        </ErrorBoundary>
      </Transition>

      <div class="divider"></div>

      <Transition fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
          {move || {
              unselected_songs_resource()
                  .map(move |songs| match songs {
                      Err(e) => {
                          view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                              .into_view()
                      }
                      Ok(s) => {
                          view! { <GigUnselectedSongListView songs=s.1 gig_id=s.0 add_song/> }
                              .into_view()
                      }
                  })
                  .unwrap_or_default()
          }}

        </ErrorBoundary>
      </Transition>
    }
}

type Act<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn GigSongListView(
    songs: Vec<SongKind>,
    gig_id: i32,
    remove_song: Act<RemoveSongFromGig>,
    move_song: Act<MoveSongInGig>,
) -> impl IntoView {
    view! {
      <Suspense fallback=move || view! { <div></div> }>
        <div>
          <div class="grid grid-cols-8 gap-2">
              {
        let mut songs_indexed: Vec<(usize, SongKind)> = Vec::default();
        let mut break_count = 0usize;

        for song in songs.iter().enumerate() {
            if let SongKind::Break(_) = song.1 {
                break_count += 1;
                songs_indexed.push((0, song.1.clone()));
            } else {
                songs_indexed.push((song.0 - break_count, song.1.clone()));
            }
        }
              songs_indexed
                  .into_iter()
                  .map(move |(index, song)| view! { <SelectedGigSong index song gig_id remove_song move_song/> })
                  .collect_view()}
          </div>
        </div>
      </Suspense>
    }
}

#[component]
pub fn GigUnselectedSongListView(
    songs: Vec<Song>,
    gig_id: i32,
    add_song: Act<AddSongToGig>,
) -> impl IntoView {
    view! {
      <Suspense fallback=move || view! { <div></div> }>
        <div>
          <table class="table table-xs">
            <thead></thead>
            <tbody>
              {
        songs
                  .clone()
                  .into_iter()
                  .enumerate()
                  .map(move |(index, song)| view! { <UnSelectedGigSong index song gig_id add_song/> })
                  .collect_view()}
            </tbody>
          </table>
        </div>
      </Suspense>
    }
}

#[component]
pub fn SelectedGigSong(
    index: usize,
    song: SongKind,
    gig_id: i32,
    remove_song: Act<RemoveSongFromGig>,
    move_song: Act<MoveSongInGig>,
) -> impl IntoView {
    let edit_mode = use_context::<RwSignal<bool>>().unwrap();

    fn buttons(
        song_id: i32,
        gig_id: i32,
        remove_song: Act<RemoveSongFromGig>,
        move_song: Act<MoveSongInGig>,
    ) -> impl IntoView {
        let edit_mode = use_context::<RwSignal<bool>>().unwrap();
        move || match edit_mode() {
            true => view! {
              <div class="col-span-1">
                <button
                  type="button"
                  class="btn btn-secondary btn-circle btn-outline"
                  on:click=move |_| {
                      remove_song
                          .dispatch(RemoveSongFromGig {
                              gig_id: gig_id,
                              song_id: song_id,
                          })
                  }
                >

                  <i class="fa-solid fa-minus"></i>
                </button>
              </div>
              <div class="col-span-2">
                <div class="join">
                  <button
                    type="button"
                    class="btn btn-neutral join-item"
                    on:click=move |_| {
                        move_song
                            .dispatch(MoveSongInGig {
                                gig_id: gig_id,
                                song_id: song_id,
                                kind: MoveKind::Up,
                            })
                    }
                  >

                    <i class="fa-solid fa-chevron-up"></i>
                  </button>
                  <button
                    type="button"
                    class="btn btn-neutral join-item"
                    on:click=move |_| {
                        move_song
                            .dispatch(MoveSongInGig {
                                gig_id: gig_id,
                                song_id: song_id,
                                kind: MoveKind::Down,
                            })
                    }
                  >

                    <i class="fa-solid fa-chevron-down"></i>
                  </button>
                </div>
              </div>
            }
            .into_view(),
            false => view! {}.into_view(),
        }
    }

    view! {
      {
        let pauze_cols = move || match edit_mode() {
            true => "col-start-2 col-span-4 font-bold",
            false => "col-start-2 col-span-7 font-bold"
        };
        let song_cols = move || match edit_mode() {
            true => "col-span-4",
            false => "col-span-7"
        };
        match song {
          SongKind::Break(break_id) => {
              view! {
                  <div class={pauze_cols}>"pauze"</div>
                  {buttons(break_id, gig_id, remove_song, move_song)}
              }
                  .into_view()
          }
          SongKind::Song(s) => {
              view! {
                  <div class="font-bold col-span-1 pl-5">{index + 1}</div>
                  <div class={song_cols}>
                    <SongItem song=s.clone()/>
                  </div>
                  {buttons(s.id, gig_id, remove_song, move_song)}
              }
                  .into_view()
          }
      }}
    }
}

#[component]
pub fn UnSelectedGigSong(
    index: usize,
    song: Song,
    gig_id: i32,
    add_song: Act<AddSongToGig>,
) -> impl IntoView {
    view! {
      <tr>
        <td>
          <SongItem song=song.clone()/>
        </td>
        <td>
          <button
            type="button"
            class="btn btn-primary btn-outline btn-circle"
            on:click=move |_| {
                add_song
                    .dispatch(AddSongToGig {
                        gig_id: gig_id,
                        song_id: song.id,
                    })
            }
          >

            <i class="fa-solid fa-plus"></i>
          </button>

        </td>
      </tr>
    }
}
