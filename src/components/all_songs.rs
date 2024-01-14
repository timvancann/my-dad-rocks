use leptos::*;

use crate::models::song::Song;

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

#[component]
pub fn AllSongs(set_song_id: WriteSignal<Option<i32>>) -> impl IntoView {
    let all_songs = create_resource(|| (), |_| async move { get_songs().await });

    provide_context(set_song_id);
    provide_context(all_songs);

    fn songs_to_view(songs: Vec<Song>) -> impl IntoView {
        view! {
        <section class="flex flex-col items-center justify-center">
          <table class="w-fit text-sm text-gray-500 dark:text-gray-400 shadow-md">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th scope="col" class="px-3 py-1"></th>
                <th scope="col" class="px-3 py-1">Artiest</th>
                <th scope="col" class="px-3 py-1">Titel</th>
                <th scope="col" class="px-3 py-1">Laatst gespeeld</th>
                <th scope="col" class="px-3 py-1"></th>
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
        </section>
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
      <tr class="odd:bg-white odd:dark:bg-gray-900 even:bg-gray-50 even:dark:bg-gray-800 border-b dark:border-gray-700">
        <td class="px-3 py-1">
        <button type="button"
            class="text-white bg-blue-500 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-md text-sm px-5 py-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 transition-all"
            on:click=move |_| {
                set_played.update(|id| *id = Some(song.id));
            }
          >

            <i class="fa fa-play"></i>
          </button>
        </td>
        <td class="px-3 py-1">{song.artist}</td>
        <th class="px-3 py-1 font-medium text-gray-900 whitespace-nowrap dark:text-white">{song.title}</th>
        <td class="px-3 py-1">
          {match song.last_played_at {
              Some(d) => view! { <div>{d.format("%d-%m-%Y").to_string()}</div> },
              None => view! { <div>"Nooit"</div> },
          }}

        </td>
        <td class="px-3 py-1">
        <button type="button"
            class="text-white bg-yellow-600 hover:bg-yellow-900 focus:ring-4 focus:outline-none focus:ring-yellow-300 font-medium rounded-md text-sm px-5 py-2.5 text-center inline-flex items-center me-2 dark:bg-yellow-600 dark:hover:bg-yellow-700 dark:focus:ring-yellow-800 transition-all"
            on:click=move |_| {
                set_played_action.dispatch((song.id, song_getter));
            }
          >

            <i class="fa-solid fa-check"></i>
          </button>
        </td>
      </tr>
    }
}
