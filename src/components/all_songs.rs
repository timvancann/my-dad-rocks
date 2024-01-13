use leptos::*;

use crate::models::song::Song;

use super::random_selection::get_random_song;

#[server(GetSongs)]
pub async fn get_songs() -> Result<Vec<Song>, ServerFnError> {
    match Song::get_all().await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<(), ServerFnError> {
    logging::log!("Update song played");
    match Song::set_played(song_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[cfg(feature = "ssr")]
async fn find_unpracticed_unselected_random_song(
    already_selected: Vec<Song>,
) -> Result<Song, ServerFnError> {
    while let Ok(song) = get_random_song().await {
        if !song.practice_next && !already_selected.contains(&song) {
            return Ok(song);
        }
    }
    return Err(ServerFnError::ServerError(
        "Can't find unpracticed songs".to_string(),
    ));
}

#[cfg(feature = "ssr")]
async fn reset_song_practice() -> Result<(), ServerFnError> {
    let songs = Song::get_all().await?;
    for song in songs {
        song.set_to_practice_next(false).await?;
    }
    Ok(())
}

#[server(SelectRandomSongs)]
pub async fn select_next_songs_to_practice(n: i32) -> Result<(), ServerFnError> {
    reset_song_practice().await?;
    let mut selected: Vec<Song> = Vec::default();
    for _ in 0..n {
        let song = find_unpracticed_unselected_random_song(selected.clone()).await?;
        song.set_to_practice_next(true).await?;
        selected.push(song);
    }
    Ok(())
}

#[component]
pub fn AllSongs(set_song_id: WriteSignal<Option<i32>>) -> impl IntoView {
    let all_songs = create_resource(|| (), |_| async move { get_songs().await });

    let practice_action = create_action(
        |input: &(i32, Resource<(), Result<Vec<Song>, ServerFnError>>)| {
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
    provide_context(all_songs);

    fn songs_to_view(songs: Vec<Song>) -> impl IntoView {
        view! {
          <table class="table table-striped">
            <thead>
              <tr>
                <th scope="col"></th>
                <th scope="col">Artiest</th>
                <th scope="col">Titel</th>
                <th scope="col">Laatst gespeeld</th>
              </tr>
            </thead>
            <tbody>
              <For
                each=move || songs.clone().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(_, song)| {
                    view! { <SongView song/> }
                }
              />

            </tbody>
          </table>
        }
    }

    let songs_view = move || match all_songs.get() {
        Some(result) => match result {
            Ok(song) => view! { <div>{songs_to_view(song)}</div> },
            Err(_) => view! { <div></div> },
        },
        None => view! { <div></div> },
    };

    view! {
      <h2 class="text-center display-7 text-dark">Alle nummers</h2>
      <button
        class="btn btn-success"
        on:click=move |_| {
            practice_action.dispatch((4, all_songs));
        }
      >

        <i class="bi bi-arrow-clockwise"></i>
        Oefenen
      </button>
      <Suspense fallback=move || {
          view! { <div>"Loading.."</div> }
      }>{songs_view}</Suspense>
    }
}

#[component]
pub fn SongView(song: Song) -> impl IntoView {
    let song_getter = use_context::<Resource<(), Result<Vec<Song>, ServerFnError>>>()
        .expect("to have the getter provided");

    let set_played_action = create_action(
        |input: &(i32, Resource<(), Result<Vec<Song>, ServerFnError>>)| {
            let input = input.to_owned();
            async move {
                set_song_played(input.0).await.map(|res| {
                    input.1.refetch();
                    res
                })
            }
        },
    );

    let set_played =
        use_context::<WriteSignal<Option<i32>>>().expect("to have found the setter provided");

    view! {
      <tr class:table-primary=move || { song.practice_next }>
        <td>
          <button
            class="btn btn-info"
            on:click=move |_| {
                set_played.update(|id| *id = Some(song.id));
            }
          >

            <i class="bi bi-play-circle-fill"></i>
          </button>
        </td>
        <td>{song.artist}</td>
        <th>{song.title}</th>
        <td>
          {match song.last_played_at {
              Some(d) => view! { <div>{d.format("%d-%m-%Y").to_string()}</div> },
              None => view! { <div>"Nooit"</div> },
          }}

        </td>
        <td>
          <button
            class="btn btn-success"
            on:click=move |_| {
                set_played_action.dispatch((song.id, song_getter));
            }
          >

            <i class="bi bi-check-circle"></i>
          </button>
        </td>
      </tr>
    }
}
