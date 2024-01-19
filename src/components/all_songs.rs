use leptos::*;

use crate::{
    components::song::SongListView,
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

#[server(SelectRandomSongs)]
pub async fn select_next_songs_to_practice(max_n: i32) -> Result<(), ServerFnError> {
    let setlist = get_setlist().await?;
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

#[server(GetSetlist)]
pub async fn get_setlist() -> Result<Setlist, ServerFnError> {
    match Setlist::get().await {
        Ok(setlist) => Ok(setlist),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(CleanSetlist)]
pub async fn clean_setlist() -> Result<(), ServerFnError> {
    Setlist::clean().await.map_err(ServerFnError::from)
}

#[component]
pub fn AllSongs() -> impl IntoView {
    let songs_resource = create_resource(|| (), |_| async move { get_songs().await });
    let setlist_resource = create_resource(|| (), |_| async move { get_setlist().await });

    let setlist_songs = move || match (songs_resource(), setlist_resource()) {
        (Some(Ok(songs)), Some(Ok(setlist))) => Some(
            songs
                .into_iter()
                .filter(|s| setlist.songs.contains(&s.id))
                .collect::<Vec<_>>(),
        ),
        _ => None,
    };

    let practice_action = create_action(
        |input: &(i32, Resource<(), Result<Setlist, ServerFnError>>)| {
            let input = input.to_owned();
            async move {
                select_next_songs_to_practice(input.0).await.map(|res| {
                    input.1.refetch();
                    res
                })
            }
        },
    );

    let clear_setlist_action =
        create_action(|input: &Resource<(), Result<Setlist, ServerFnError>>| {
            let input = input.to_owned();
            async move {
                clean_setlist().await.map(|res| {
                    input.refetch();
                    res
                })
            }
        });

    view! {
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Setlist</div>
        <div class="join">
          <button
            type="button"
            class="btn btn-accent btn-outline join-item"
            on:click=move |_| { practice_action.dispatch((4, setlist_resource)) }
          >
            <i class="fa-solid fa-rotate-right"></i>
            Vul setlist
          </button>
          <button
            type="button"
            class="btn btn-accent btn-outline join-item"
            on:click=move |_| { clear_setlist_action.dispatch(setlist_resource) }
          >
            <i class="fa-solid fa-trash"></i>
            Leeg setlist
          </button>
                </div>
      </div>
      <div>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
          {match setlist_songs() {
              Some(songs) => {
                  view! {
                    <div>
                      <SongListView songs songs_resource setlist_resource/>
                    </div>
                  }
              }
              None => view! { <div>Geen nummers</div> },
          }}

        </Suspense>
      </div>
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Alle nummers</div>
      </div>
      <div>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
          {match songs_resource() {
              Some(Ok(songs)) => {
                  view! {
                    <div>
                      <SongListView songs songs_resource setlist_resource/>
                    </div>
                  }
              }
              _ => view! { <div>Geen nummers</div> },
          }}

        </Suspense>
      </div>
    }
}
