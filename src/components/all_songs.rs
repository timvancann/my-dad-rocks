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
pub async fn select_next_songs_to_practice(n: i32) -> Result<(), ServerFnError> {
    let mut selected: Vec<Song> = Vec::default();
    for _ in 0..n {
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

#[component]
pub fn AllSongs() -> impl IntoView {
    let songs_resource = create_resource(|| (), |_| async move { get_songs().await });
    let setlist = create_resource(|| (), |_| async move { get_setlist().await });

    let setlist_songs = move || match (songs_resource(), setlist()) {
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

    view! {
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Setlist</div>
        <button
          type="button"
          class="btn btn-accent btn-outline"
          on:click=move |_| { practice_action.dispatch((4, setlist)) }
        >

          <i class="bi bi-arrow-clockwise"></i>
          Verander setlist
        </button>
      </div>
      <div>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
          {match setlist_songs() {
              Some(songs) => {
                  view! {
                    <div>
                      <SongListView songs songs_resource/>
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
                      <SongListView songs songs_resource/>
                    </div>
                  }
              }
              _ => view! { <div>Geen nummers</div> },
          }}

        </Suspense>
      </div>
    }
}
