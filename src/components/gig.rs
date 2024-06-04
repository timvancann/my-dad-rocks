use leptos::*;
use leptos_router::*;

use crate::{
    components::song_item::SongItem,
    models::gig::{Gig, MoveKind, SongKind},
};
use crate::components::player::{Player, PlayerData};
use crate::components::shared::{Horizontal, LyricsButton, PlayButton};
use crate::models;
use crate::models::song::Song;

#[server(GetGig, "/api", "GetJson")]
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

#[server(SetGigVenue)]
async fn set_gig_venue(gig_id: i32, venue: String) -> Result<(), ServerFnError> {
    Gig::set_venue(gig_id, venue)
        .await
        .map_err(ServerFnError::from)
}

#[server(SetGigTime)]
async fn set_gig_time(gig_id: i32, time: String) -> Result<(), ServerFnError> {
    Gig::set_time(gig_id, time)
        .await
        .map_err(ServerFnError::from)
}

#[server(SetGigDate)]
async fn set_gig_date(gig_id: i32, date: String) -> Result<(), ServerFnError> {
    Gig::set_date(gig_id, date)
        .await
        .map_err(ServerFnError::from)
}

#[server(RemoveGig)]
pub async fn remove_gig(gig_id: i32) -> Result<(), ServerFnError> {
    let res = Gig::remove(gig_id).await.map_err(ServerFnError::from);
    leptos_axum::redirect("/gigs");
    res
}

#[derive(Params, PartialEq)]
struct GigParams {
    id: Option<usize>,
}

#[component]
pub fn Gig() -> impl IntoView {
    let (get_player_data, set_player_data) = create_signal::<Option<PlayerData>>(None);
    provide_context(set_player_data);
    provide_context(get_player_data);

    let (get_selected_song, set_selected_song) = create_signal::<Option<i32>>(None);
    provide_context(get_selected_song);
    provide_context(set_selected_song);

    let params = use_params::<GigParams>();
    let gig_id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.unwrap_or_default())
                .unwrap_or_default() as i32
        })
    };

    let remove_song = create_server_action::<RemoveSongFromGig>();
    let add_song = create_server_action::<AddSongToGig>();
    let move_song = create_server_action::<MoveSongInGig>();

    let gig_resource = create_resource(
        move || {
            (
                remove_song.version().get(),
                add_song.version().get(),
                move_song.version().get(),
                Some(gig_id()),
            )
        },
        |args| get_gig(args.3),
    );

    view! {
      <Player/>
      <div class="flex flex-col mt-2">
        <div class="flex gap-2 mx-2">
          <Transition>

            {
                let gig = gig_resource
                    .get()
                    .unwrap_or_else(|| Ok(Gig::default()))
                    .unwrap_or_default();
                view! {
                  <MetaDataButtons gig/>
                  <DeleteGigButton gig_id=gig_id()/>
                }
            }

          </Transition>
        </div>
      </div>

      <Horizontal/>

      <div class="grid grid-flow-row auto-rows-max gap-2">
        <Transition>
          <For
            each=move || {
                gig_resource
                    .get()
                    .unwrap_or_else(|| Ok(Gig::default()))
                    .unwrap_or_default()
                    .songs
                    .into_iter()
            }

            key=|state| state.clone()
            let:song
          >
            <SelectedGigSong selected_song=song gig_id=gig_id() remove_song move_song/>
          </For>
        </Transition>
      </div>

      <Horizontal/>
      <div class="flex justify-end mr-2 mt-4 mb-2">
        <PauseButton gig_id=gig_id() add_song/>
      </div>

      <div class="mx-2">
        <Transition>
          <For
            each=move || {
                gig_resource
                    .get()
                    .unwrap_or_else(|| Ok(Gig::default()))
                    .unwrap_or_default()
                    .unselected_songs
                    .into_iter()
            }

            key=|state| state.clone()
            let:song
          >
            <UnSelectedGigSong song gig_id=gig_id() add_song/>
          </For>

        </Transition>
      </div>
    }
}

#[component]
pub fn MetaDataButtons(gig: Gig) -> impl IntoView {
    let set_gig_venue = create_server_action::<SetGigVenue>();
    let set_gig_time = create_server_action::<SetGigTime>();
    let set_gig_date = create_server_action::<SetGigDate>();

    view! {
      <InputWithLabel
        label="Venue".to_string()
        value=gig.venue
        on:input=move |ev| {
            set_gig_venue
                .dispatch(SetGigVenue {
                    gig_id: gig.id,
                    venue: event_target_value(&ev),
                });
        }
      />

      <InputWithLabel
        label="Tijd".to_string()
        value=gig.time.unwrap_or("".to_string())
        on:input=move |ev| {
            set_gig_time
                .dispatch(SetGigTime {
                    gig_id: gig.id,
                    time: event_target_value(&ev),
                });
        }
      />

      <InputWithLabel
        label="Datum".to_string()
        value=gig.date.to_string()
        on:input=move |ev| {
            set_gig_date
                .dispatch(SetGigDate {
                    gig_id: gig.id,
                    date: event_target_value(&ev),
                });
        }
      />
    }
}

#[component]
pub fn DeleteGigButton(gig_id: i32) -> impl IntoView {
    let remove_gig = create_server_action::<RemoveGig>();

    view! {
      <div class="self-end">
        <button
          type="submit"
          class="border-0 rounded-full px-3 py-2 shadow-lg bg-ctp-maroon text-ctp-mantle"
          on:click=move |_| {
              remove_gig.dispatch(RemoveGig { gig_id });
              use_navigate()("/gigs", Default::default());
          }
        >

          <i class="fa-solid fa-trash"></i>
        </button>
      </div>
    }
}

type Act<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn PauseButton(gig_id: i32, add_song: Act<AddSongToGig>) -> impl IntoView {
    view! {
      <button
        type="button"
        class="border-0 rounded-full px-3 py-2 shadow-md bg-ctp-teal text-ctp-mantle"
        on:click=move |_| {
            add_song
                .dispatch(AddSongToGig {
                    gig_id,
                    song_id: -1,
                })
        }
      >

        <i class="fa-solid fa-pause"></i>
        Pauze
      </button>
    }
}

#[component]
pub fn MoveSongInSet(
    song_id: i32,
    gig_id: i32,
    move_song: Act<MoveSongInGig>,
    direction: MoveKind,
) -> impl IntoView {
    let chevron = match direction {
        MoveKind::Up => "fa-chevron-up",
        MoveKind::Down => "fa-chevron-down",
    };
    view! {
      <button
        type="button"
        class="border-0 rounded-md px-3 py-2 ml-4 w-16 shadow-md bg-ctp-lavender text-ctp-mantle"
        on:click=move |_| {
            move_song
                .dispatch(MoveSongInGig {
                    gig_id,
                    song_id,
                    kind: direction,
                })
        }
      >

        <i class=format!("fa-solid {}", chevron)></i>
      </button>
    }
}

#[component]
pub fn SelectedGigSong(
    selected_song: (usize, SongKind),
    gig_id: i32,
    remove_song: Act<RemoveSongFromGig>,
    move_song: Act<MoveSongInGig>,
) -> impl IntoView {
    let get_selected_song = use_context::<ReadSignal<Option<i32>>>()
        .expect("Expected to have a selected song signal provided");
    let set_selected_song = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a selected song signal provided");
    view! {
      {match selected_song.1 {
          SongKind::Break(break_id) => {
              view! {
                <div class="flex flow-row justify-between">
                  <button
                    on:click=move |_| {
                        set_selected_song
                            .update(|id| {
                                *id = if *id == Some(break_id) { None } else { Some(break_id) };
                            });
                    }

                    class="flex-1"
                  >
                    <div class="place-self-center font-bold text-sm ml-6 my-3">pauze</div>
                  </button>
                </div>
                <Show when=move || get_selected_song.get() == Some(break_id)>
                  <div class="ml-2 flex mt-2">
                    <div class="flex items-center flex-1"></div>
                    <div class="flex items-center mr-2">
                      <RemoveSongButton song_id=break_id gig_id remove_song/>
                      <MoveSongInSet song_id=break_id gig_id move_song direction=MoveKind::Up/>
                      <MoveSongInSet song_id=break_id gig_id move_song direction=MoveKind::Down/>
                    </div>
                  </div>
                </Show>
              }
                  .into_view()
          }
          SongKind::Song(song) => {
              view! {
                <div class="bg-ctp-crust py-2 rounded-lg border-0 shadow-md">
                  <div class="ml-2 flex">
                    <div class="flex">
                      <div class="font-medium text-xs self-center mr-2 justify-self-end">
                        {selected_song.0 + 1}
                      </div>
                    </div>
                    <button
                      on:click=move |_| {
                          set_selected_song
                              .update(|id| {
                                  *id = if *id == Some(song.id) { None } else { Some(song.id) };
                              });
                      }

                      class="flex-1"
                    >
                      <SongItem song=song.clone()/>
                    </button>
                    <div class="flex items-center mr-2">
                      // todo tvc: fix setlist_id / gig_id confusion
                      <PlayButton song_id=song.id setlist_id=1/>
                    </div>
                  </div>
                  <Show when=move || get_selected_song.get() == Some(song.id)>
                    <div class="ml-2 flex mt-2">
                      <div class="flex items-center flex-1">
                        <LyricsButton song_id=song.id/>
                      </div>
                      <div class="flex items-center mr-2">
                        <RemoveSongButton song_id=song.id gig_id remove_song/>
                        <MoveSongInSet song_id=song.id gig_id move_song direction=MoveKind::Up/>
                        <MoveSongInSet song_id=song.id gig_id move_song direction=MoveKind::Down/>
                      </div>
                    </div>
                  </Show>
                </div>
              }
                  .into_view()
          }
      }}
    }
}

#[component]
pub fn RemoveSongButton(
    song_id: i32,
    gig_id: i32,
    remove_song: Act<RemoveSongFromGig>,
) -> impl IntoView {
    view! {
      <button
        type="button"
        class="border-0 rounded-full px-3 py-2 w-16 shadow-md bg-ctp-maroon text-ctp-mantle"
        on:click=move |_| {
            remove_song
                .dispatch(RemoveSongFromGig {
                    gig_id,
                    song_id,
                })
        }
      >

        <i class="fa-solid fa-minus"></i>
      </button>
    }
}

#[component]
pub fn UnSelectedGigSong(song: Song, gig_id: i32, add_song: Act<AddSongToGig>) -> impl IntoView {
    view! {
      <div class="flex justify-between items-center">
        <SongItem song=song.clone()/>
        <button
          type="button"
          class="border-0 rounded-full px-3 py-2 shadow-md bg-ctp-teal text-ctp-mantle"
          on:click=move |_| {
              add_song
                  .dispatch(AddSongToGig {
                      gig_id,
                      song_id: song.id,
                  })
          }
        >

          <i class="fa-solid fa-plus"></i>
        </button>
      </div>
    }
}

#[component]
pub fn InputWithLabel(label: String, value: String) -> impl IntoView {
    view! {
      <div>
        <label class="block text-sm font-medium leading-6">{label}</label>
        <div class="relative mt-2 rounded-md shadow-sm text-sm">
          <input
            type="text"
            class="input block w-full rounded-md border-0 py-2 pl-2 text-ctp-mantle ring-1 ring-inset ring-ctp-surface0 focus:ring-2 focus:ring-inset focus:ring-ctp-flamingo"
            prop:value=value
            placeholder="Venue"
          />
        </div>
      </div>
    }
}
