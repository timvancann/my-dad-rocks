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
          <table class="table">
            <thead></thead>
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
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Alle nummers</div>
      </div>
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
      <tr>
        <td>
          <div class="flex items-center gap-3">
            <div class="avatar">
              <div class="mask mask-squircle w-12 h-12"></div>
            </div>
            <div>
              <div class="font-bold">{song.title}</div>
              <div class="text-sm opacity-70">{song.artist}</div>
              <div class="badge badge-outline text-sm opacity-50">
          {match song.last_played_at {
              Some(d) => d.format("%d-%m-%Y").to_string(),
              None => "Nooit".to_string(),
          }}
              </div>
            </div>
          </div>
        </td>
        <td>
          <button
            type="button"
            class="btn btn-primary btn-circle"
            on:click=move |_| {
                set_played.update(|id| *id = Some(song.id));
            }
          >

            <i class="fa fa-play"></i>
          </button>
        </td>
        <td>
          <button
            type="button"
            class="btn btn-outline btn-error"
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
