use crate::components::random_selection::get_random_song;
use crate::components::song::SongListView;
use crate::models::{setlist::Setlist, song::Song};
use leptos::*;

#[server(GetSetlist)]
pub async fn get_setlist() -> Result<Setlist, ServerFnError> {
    match Setlist::get().await {
        Ok(setlist) => Ok(setlist),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[cfg(feature = "ssr")]
pub async fn find_unpracticed_unselected_random_song(
    already_selected: &Vec<Song>,
) -> Result<Song, ServerFnError> {
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

#[component]
pub fn SetlistView() -> impl IntoView {
    let setlist_resource = create_resource(|| (), |_| async move { get_setlist().await });

    let setlist_songs = create_resource(
        || (),
        |_| async move {
            match get_setlist().await {
                Ok(sl) => Ok(sl.songs),
                Err(e) => Err(e),
            }
        },
    );
    provide_context(setlist_songs);

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
      <div>
        <div class="flex items-center justify-between mb-4 ml-4 mr-4">
          <div class="font-bold text-2xl">Setlist</div>
          <button
            type="button"
            class="btn btn-accent btn-outline"
            on:click=move |_| { practice_action.dispatch((4, setlist_resource)) }
          >

            <i class="bi bi-arrow-clockwise"></i>
            Verander setlist
          </button>
        </div>
        <SongListView/>

      </div>
    }
}
