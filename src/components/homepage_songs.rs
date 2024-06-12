use leptos::*;
use leptos_router::ActionForm;

use crate::components::shared::{EditButton, Horizontal, LyricsButton, PlayButton};
use crate::components::song_item::SongItem;
use crate::models::setlist::Setlist;
use crate::models::song::{Rehearsal, Song};

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
pub async fn get_songs() -> Result<Rehearsal> {
    match Song::get_rehearsal().await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(CleanSetlist)]
pub async fn clean_setlist() -> Result<()> {
    Setlist::clean().await.map_err(ServerFnError::from)
}

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<()> {
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

    let (get_selected_song, set_selected_song) = create_signal::<Option<i32>>(None);

    let rehearsal = create_resource(
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

    view! {
      <div class="flex justify-between m-3 items-center">
        <div class="font-bold text-xl flex">Setlist</div>
        <div class="flex">
          <FillButton fill_action=fill/>
          <CleanButton clean_action=empty_setlist/>
        </div>
      </div>
      <div class="grid grid-flow-row auto-rows-max gap-2">
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>

          <For
            each=move || {
                rehearsal
                    .get()
                    .unwrap_or_else(|| Ok(Rehearsal::default()))
                    .unwrap_or_default()
                    .selected_songs
                    .into_iter()
            }

            key=|state| state.clone()
            let:song
          >
            <SongView
              song
              all_songs=rehearsal
                  .get()
                  .unwrap_or_else(|| Ok(Rehearsal::default()))
                  .unwrap_or_default()
                  .selected_songs
              pick_song
              in_rehearsal=true
              set_song_played
              get_selected_song
              set_selected_song
            />
          </For>

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
            each=move || {
                rehearsal
                    .get()
                    .unwrap_or_else(|| Ok(Rehearsal::default()))
                    .unwrap_or_default()
                    .unselected_songs
                    .into_iter()
            }

            key=|state| state.clone()
            let:song
          >
            <SongView
              song
              all_songs=rehearsal
                  .get()
                  .unwrap_or_else(|| Ok(Rehearsal::default()))
                  .unwrap_or_default()
                  .unselected_songs
              pick_song
              in_rehearsal=false
              set_song_played
              get_selected_song
              set_selected_song
            />
          </For>
        </Transition>
      </div>
    }
}

#[component]
pub fn FillButton(fill_action: Action<FillSetlist, Result<()>>) -> impl IntoView {
    view! {
      <ActionForm action=fill_action>
        <input type="number" hidden=true name="max_n" value=4/>

        <button
          type="submit"
          class="border-0 border-md rounded-l-lg mr-1 px-2 py-1 shadow-md bg-ctp-teal text-ctp-mantle"
        >
          <i class="fa-solid fa-rotate-right"></i>
          Vullen
        </button>
      </ActionForm>
    }
}

#[component]
pub fn CleanButton(clean_action: Action<CleanSetlist, Result<()>>) -> impl IntoView {
    view! {
      <ActionForm action=clean_action>
        <button
          type="submit"
          class="border-0 border-md rounded-r-lg px-2 py-1 shadow-md bg-ctp-teal text-ctp-mantle"
        >
          <i class="fa-solid fa-trash"></i>
          Legen
        </button>
      </ActionForm>
    }
}

#[component]
pub fn SongView(
    song: Song,
    all_songs: Vec<Song>,
    pick_song: Action<HandPickSong, Result<()>>,
    in_rehearsal: bool,
    set_song_played: Action<SetSongPlayed, Result<()>>,
    get_selected_song: ReadSignal<Option<i32>>,
    set_selected_song: WriteSignal<Option<i32>>,
) -> impl IntoView {
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
            <PlayButton song=song.clone() all_songs/>
          </div>
        </div>
        <Show when=move || get_selected_song.get() == Some(song.id)>
          <div class="ml-2 flex">
            <div class="flex-1 items-center mr-2 mt-1 mb-1">
              <LyricsButton song_id=song.id/>
              <EditButton song_id=song.id/>
              <Show when=move || !in_rehearsal>
                <ActionForm action=pick_song class="inline">
                  <input type="number" hidden=true name="song_id" value=song.id/>
                  <button
                    type="submit"
                    class="border-0 rounded-md ml-2 px-3 py-2 shadow-md bg-ctp-lavender text-ctp-mantle inline"
                  >

                    <i class="fa-solid fa-arrow-up"></i>
                    Oefen
                  </button>
                </ActionForm>
              </Show>
            </div>
            <div class="flex justify-end mr-2 items-center">
              <button
                type="button"
                class="border-0 rounded-md ml-2 px-3 py-2 shadow-md bg-ctp-flamingo text-ctp-mantle text-xs"
                on:click=move |_| { set_song_played.dispatch(SetSongPlayed { song_id: song.id }) }
              >

                <i class="fa-solid fa-music"></i>
                " "
                {match song.last_played_at {
                    Some(d) => d.format("%d-%m-%Y").to_string(),
                    None => "Nooit".to_string(),
                }}

              </button>
            </div>
          </div>
        </Show>

      </div>
    }
}
