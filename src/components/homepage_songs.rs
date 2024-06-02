use leptos::*;

use crate::components::shared::Horizontal;
use crate::components::song_item::SongItem;
use crate::models::setlist::Setlist;
use crate::models::song::Song;

type Result<T> = std::result::Result<T, ServerFnError>;

#[cfg(feature = "ssr")]
pub async fn find_unpracticed_unselected_random_song(already_selected: &[Song]) -> Result<Song> {
    use super::random_selection::get_random_song;

    while let Ok(song) = get_random_song().await {
        if !already_selected.contains(&song) {
            return Ok(song);
        }
    }
    Err(ServerFnError::ServerError(
        "Can't find unpracticed songs".to_string(),
    ))
}

#[server(FillSetlist)]
pub async fn fill_setlist(max_n: i32) -> Result<()> {
    let setlist = Setlist::get().await?;
    let songs_to_find = max_n - setlist.songs.len() as i32;
    let mut selected: Vec<Song> = Vec::default();
    for _ in 0..songs_to_find {
        let song = find_unpracticed_unselected_random_song(&selected).await?;
        selected.push(song);
    }

    Setlist::set_songs(selected.iter().map(|s| s.id).collect())
        .await
        .map_err(ServerFnError::from)
}

#[server(GetSongs, "/api", "GetJson")]
pub async fn get_songs() -> Result<Vec<Song>> {
    match Song::get_all().await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(CleanSetlist, "/api", "GetJson")]
pub async fn clean_setlist() -> Result<()> {
    Setlist::clean().await.map_err(ServerFnError::from)
}

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<()> {
    logging::log!("Update song played");
    match Song::set_played(song_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(HandPickSong)]
pub async fn pick_song(song_id: i32) -> Result<()> {
    Setlist::set_songs(vec![song_id])
        .await
        .map_err(ServerFnError::from)
}

#[component]
pub fn Songs() -> impl IntoView {
    let empty_setlist = create_server_action::<CleanSetlist>();
    let fill = create_server_action::<FillSetlist>();

    let set_song_played = create_server_action::<SetSongPlayed>();
    let pick_song = create_server_action::<HandPickSong>();

    let songs_resource = create_resource(
        move || {
            (
                set_song_played.version().get(),
                pick_song.version().get(),
                empty_setlist.version().get(),
                fill.version().get(),
            )
        },
        |_| get_songs(),
    );
    let (get_selected_song, set_selected_song) = create_signal::<Option<i32>>(None);
    // let set_setlist = use_context::<WriteSignal<Vec<i32>>>()
    //     .expect("Expected to have a set_played signal provided");
    // set_setlist.update(move |s| {
    //     *s = songs_resource
    //         .get()
    //         .unwrap_or_else(|| Ok(vec![]))
    //         .unwrap_or_default()
    //         .into_iter()
    //         .filter(|s| s.should_play)
    //         .map(|s| s.id)
    //         .collect()
    // });

    view! {
      <div class="flex justify-between m-3 items-center">
        <div class="font-bold text-xl flex">Setlist</div>
        <div class="flex">

          <button
            type="submit"
            class="border-0 border-md rounded-l-lg mr-1 px-2 py-1 shadow-md bg-ctp-teal text-ctp-mantle"
            on:click=move |_| { fill.dispatch(FillSetlist { max_n: 4 }) }
          >
            <i class="fa-solid fa-rotate-right"></i>
            Vullen
          </button>

          <button

            type="button"
            class="border-0 border-md rounded-r-lg px-2 py-1 shadow-md bg-ctp-teal text-ctp-mantle"
            on:click=move |_| { empty_setlist.dispatch(CleanSetlist {}) }
          >
            <i class="fa-solid fa-trash"></i>
            Legen
          </button>

        </div>
      </div>
      <div class="grid grid-flow-row auto-rows-max gap-2">
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>

          <For
            {move || songs_resource.track()}
            each=move || {
                songs_resource
                    .get()
                    .unwrap_or_else(|| Ok(vec![]))
                    .unwrap_or_default()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, s)| s.should_play)
            }

            key=|(_, state)| state.clone()
            children=move |(index, _)| {
                let song = create_memo(move |_| {
                    songs_resource
                        .and_then(|data| { data.get(index).unwrap().clone() })
                        .unwrap_or(Ok(Song::default()))
                        .unwrap_or_default()
                });
                move || {
                    view! {
                      <SongView
                        song=song.get()
                        pick_song
                        set_song_played
                        get_selected_song
                        set_selected_song
                      />
                    }
                        .into_view()
                }
            }
          />

        </Transition>
      </div>

      <Horizontal/>

      <div class="flex items-center justify-between mb-3 ml-3">
        <div class="font-bold text-xl">Alle nummers</div>
      </div>
      <div class="grid grid-flow-row auto-rows-max gap-2">
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>

          <For
            {move || songs_resource.track()}
            each=move || {
                songs_resource
                    .get()
                    .unwrap_or_else(|| Ok(vec![]))
                    .unwrap_or_default()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, s)| !s.should_play)
            }

            key=|(_, state)| state.clone()
            children=move |(index, _)| {
                let song = create_memo(move |_| {
                    songs_resource
                        .and_then(|data| { data.get(index).unwrap().clone() })
                        .unwrap_or(Ok(Song::default()))
                        .unwrap_or_default()
                });
                move || {
                    view! {
                      <SongView
                        song=song.get()
                        pick_song
                        set_song_played
                        get_selected_song
                        set_selected_song
                      />
                    }
                        .into_view()
                }
            }
          />

        </Transition>
      </div>
    }
}

#[component]
pub fn SongView(
    song: Song,
    pick_song: Action<HandPickSong, Result<()>>,
    set_song_played: Action<SetSongPlayed, Result<()>>,
    get_selected_song: ReadSignal<Option<i32>>,
    set_selected_song: WriteSignal<Option<i32>>,
) -> impl IntoView {
    let set_song_id = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a set_played signal provided");

    view! {
      <div class="bg-ctp-crust py-2 rounded-lg border-0 shadow-md">
        <div class="ml-2 flex">
          <button
            on:click=move |_| {
                set_selected_song
                    .update(|id| *id = if *id == Some(song.id) { None } else { Some(song.id) });
            }

            class="flex-1"
          >

            <SongItem song=song.clone()/>
          </button>
          <div class="flex items-center mr-2">
            <button
              type="button"
              class="border-0 rounded-lg py-3 w-16 shadow-lg bg-ctp-green text-ctp-mantle"
              on:click=move |_| {
                  set_song_id.update(|id| *id = Some(song.id));
              }
            >

              <i class="fa fa-play"></i>
            </button>
          </div>
        </div>
        <Show when=move || get_selected_song.get() == Some(song.id)>
          <div class="ml-2 flex">
            <div class="flex-1 items-center mr-2 mt-1 mb-1">
              <a href=format!("/lyric/{}", song.id)>
                <button
                  type="button"
                  class="border-0 rounded-md px-3 py-2 shadow-lg bg-ctp-lavender text-ctp-mantle"
                >
                  <i class="fa-solid fa-align-left"></i>
                  Lyrics
                </button>
              </a>
              <button
                type="button"
                class="border-0 rounded-md ml-2 px-3 py-2 shadow-md bg-ctp-flamingo text-ctp-mantle"
                on:click=move |_| { set_song_played.dispatch(SetSongPlayed { song_id: song.id }) }
              >

                <i class="fa-solid fa-music"></i>
              </button>

              <Show when=move || !song.should_play>
                <button
                  type="button"
                  class="border-0 rounded-md ml-2 px-3 py-2 shadow-md bg-ctp-lavender text-ctp-mantle"
                  on:click=move |_| {
                      if song.should_play {
                          return;
                      }
                      pick_song.dispatch(HandPickSong { song_id: song.id })
                  }
                >

                  <i class="fa-solid fa-square-check"></i>
                </button>
              </Show>
            </div>
            <div class="flex justify-end mr-2 items-center">
        <div class="bg-ctp-flamingo rounded-full p-1 shadow-sm text-ctp-mantle">
              <div class="text-xs text-bold">
                {match song.last_played_at {
                    Some(d) => d.format("%d-%m-%Y").to_string(),
                    None => "Nooit".to_string(),
                }}

              </div>
        </div>
            </div>
          </div>
        </Show>

      </div>
    }
}
