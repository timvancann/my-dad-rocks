use crate::components::random_selection::get_random_song;
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
        .map_err(|e| ServerFnError::from(e))
}

#[component]
pub fn SetlistView(set_song_id: WriteSignal<Option<i32>>) -> impl IntoView {
    let setlist_resource = create_resource(|| (), |_| async move { get_setlist().await });

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

    provide_context(set_song_id);

    fn songs_to_view(songs: Vec<Song>) -> impl IntoView {
        view! {
        <section class="flex flex-col items-center justify-center">
          <table class="w-fit text-sm text-gray-500 dark:text-gray-400 shadow-md">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th scope="col" class="px-3 py-1"></th>
                <th scope="col" class="px-3 py-1">Artiest</th>
                <th scope="col" class="px-3 py-1">Titel</th>
              </tr>
            </thead>
            <tbody>
              <For
                each=move || songs.clone().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(_, song)| {
                    view! { <SetlistSongView song/> }
                }
              />

            </tbody>
          </table>
        </section>
        }
    }

    let songs_view = move || match setlist_resource.get() {
        Some(result) => match result {
            Ok(setlist) => view! { <div>{songs_to_view(setlist.songs)}</div> },
            Err(_) => view! { <div></div> },
        },
        None => view! { <div></div> },
    };

    view! {
       <h3 class="text-center display-7 text-dark">Setlist</h3>
        <button
        class="btn btn-success"
        on:click=move |_| {
            practice_action.dispatch((4, setlist_resource))
        }
        >

        <i class="bi bi-arrow-clockwise"></i>
        Verander setlist
      </button>
        <Suspense fallback=move || view! { <div></div> }>
          <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
          {songs_view()}
          </div>
        </Suspense>
    }
}

#[component]
pub fn SetlistSongView(song: Song) -> impl IntoView {
    let set_song_id =
        use_context::<WriteSignal<Option<i32>>>().expect("to have found the setter provided");
    view! {
      <tr class="odd:bg-white odd:dark:bg-gray-900 even:bg-gray-50 even:dark:bg-gray-800 border-b dark:border-gray-700">
        <td class="px-3 py-1">
        <button type="button" class="text-white bg-blue-500 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-md text-sm px-5 py-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 transition-all"
            on:click=move |_| {
                set_song_id.update(|id| *id = Some(song.id));
            }
          >
            <i class="fa fa-play"></i>
          </button>
        </td>
        <td class="px-3 py-1">{song.artist}</td>
        <th class="px-3 py-1 font-medium text-gray-900 whitespace-nowrap dark:text-white">{song.title}</th>
      </tr>
    }
}
