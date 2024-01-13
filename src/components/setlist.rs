use leptos::*;

use crate::models::{setlist::Setlist, song::Song};
#[server(GetSetlist)]
async fn get_setlist() -> Result<Setlist, ServerFnError> {
    match Setlist::get().await {
        Ok(setlist) => Ok(setlist),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[component]
pub fn SetlistView(set_song_id: WriteSignal<Option<i32>>) -> impl IntoView {
    let setlist_resource = create_resource(|| (), |_| async move { get_setlist().await });

    provide_context(set_song_id);

    fn songs_to_view(songs: Vec<Song>) -> impl IntoView {
        view! {
          <table class="table table-striped">
            <thead>
              <tr>
                <th scope="col"></th>
                <th scope="col">Artiest</th>
                <th scope="col">Titel</th>
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
        <Suspense fallback=move || view! { <div></div> }>
          {songs_view()}
        </Suspense>
    }
}

#[component]
pub fn SetlistSongView(song: Song) -> impl IntoView {
    let set_song_id =
        use_context::<WriteSignal<Option<i32>>>().expect("to have found the setter provided");
    view! {
      <tr>
        <td>
          <button
            class="btn btn-info"
            on:click=move |_| {
                set_song_id.update(|id| *id = Some(song.id));
            }
          >

            <i class="bi bi-play-circle-fill"></i>
          </button>
        </td>
        <td>{song.artist}</td>
        <th>{song.title}</th>
      </tr>
    }
}
