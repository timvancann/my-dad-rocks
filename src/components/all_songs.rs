use leptos::*;
use leptos_meta::*;
use leptos_router::ActionForm;

use crate::{
    components::song::{SongListView, SongView},
    error_template::ErrorTemplate,
    models::{setlist::Setlist, song::Song},
};

#[cfg(feature = "ssr")]
pub async fn find_unpracticed_unselected_random_song(
    already_selected: &Vec<Song>,
) -> Result<Song, ServerFnError> {
    use super::random_selection::get_random_song;

    while let Ok(song) = get_random_song().await {
        if !already_selected.contains(&song) {
            return Ok(song);
        }
    }
    return Err(ServerFnError::ServerError(
        "Can't find unpracticed songs".to_string(),
    ));
}

#[server(FillSetlist)]
pub async fn fill_setlist(max_n: i32) -> Result<(), ServerFnError> {
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

#[server(GetSongs)]
pub async fn get_songs() -> Result<Vec<Song>, ServerFnError> {
    match Song::get_all().await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(CleanSetlist)]
pub async fn clean_setlist() -> Result<(), ServerFnError> {
    Setlist::clean().await.map_err(ServerFnError::from)
}

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<(), ServerFnError> {
    logging::log!("Update song played");
    match Song::set_played(song_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(HandPickSong)]
pub async fn pick_song(song_id: i32) -> Result<(), ServerFnError> {
    Setlist::set_songs(vec![song_id])
        .await
        .map_err(ServerFnError::from)
}

#[component]
pub fn AllSongs() -> impl IntoView {
    // get_songs actions:
    // - empty setlist
    // - fill setlist
    // - add song to setlist
    // - set_song_played

    let empty_setlist = create_server_action::<CleanSetlist>();
    let fill = create_server_action::<FillSetlist>();

    let set_song_played = create_server_action::<SetSongPlayed>();
    let pick_song = create_server_action::<HandPickSong>();

    let songs_resource = create_resource(
        move || {
            (
                empty_setlist.version().get(),
                fill.version().get(),
                set_song_played.version().get(),
                pick_song.version().get(),
            )
        },
        |_| get_songs(),
    );

    let setlist_resource = move || match songs_resource() {
        Some(res) => match res {
            Ok(songs) => Some(Ok(songs
                .into_iter()
                .filter(|s| s.is_practice)
                .collect::<Vec<_>>())),
            Err(e) => Some(Err(e)),
        },
        None => None,
    };

    view! {
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Setlist</div>
        <div class="join">
          <button
            type="submit"
            class="btn btn-accent btn-outline join-item"
            on:click=move |_| { fill.dispatch(FillSetlist { max_n: 4 }) }
          >
            <i class="fa-solid fa-rotate-right"></i>
            Vul setlist
          </button>
          <button
            type="button"
            class="btn btn-accent btn-outline join-item"
            on:click=move |_| { empty_setlist.dispatch(CleanSetlist {}) }
          >
            <i class="fa-solid fa-trash"></i>
            Leeg setlist
          </button>
        </div>
      </div>

      <Transition fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
          {move || {
              setlist_resource()
                  .map(move |songs| match songs {
                      Err(e) => {
                          view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                              .into_view()
                      }
                      Ok(songs) => {
                          view! { <SongListView songs pick_song set_song_played/> }.into_view()
                      }
                  })
                  .unwrap_or_default()
          }}

        </ErrorBoundary>
      </Transition>

      <div class="divider"></div>
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Alle nummers</div>
      </div>

      <Transition fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
          {move || {
              songs_resource()
                  .map(move |songs| match songs {
                      Err(e) => {
                          view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                              .into_view()
                      }
                      Ok(songs) => {
                          view! { <SongListView songs pick_song set_song_played/> }.into_view()
                      }
                  })
                  .unwrap_or_default()
          }}

        </ErrorBoundary>
      </Transition>
    }
}
