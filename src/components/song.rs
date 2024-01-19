use leptos::*;

use crate::{
    components::albumart::AlbumArt,
    models::{setlist::Setlist, song::Song},
};

#[server(SetSongPlayed)]
pub async fn set_song_played(song_id: i32) -> Result<(), ServerFnError> {
    logging::log!("Update song played");
    match Song::set_played(song_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(IsSongInPraciceSet)]
pub async fn is_song_in_practice_slection(song_id: i32) -> Result<bool, ServerFnError> {
    if let Ok(setlist) = Setlist::get().await {
        return Ok(setlist.songs.contains(&song_id));
    } else {
        return Ok(false);
    }
}

#[component]
pub fn SongListView(
    songs: Vec<Song>,
    songs_resource: Resource<(), Result<Vec<Song>, ServerFnError>>,
    setlist_resource: Resource<(), Result<Setlist, ServerFnError>>,
) -> impl IntoView {
    view! {
      <Suspense fallback=move || view! { <div></div> }>
        <div>
          <table class="table">
            <thead></thead>
            <tbody>
              {songs
                  .iter()
                  .map(|song| view! { <SongView song=song.clone() songs_resource setlist_resource/> })
                  .collect_view()}
            </tbody>
          </table>
        </div>
      </Suspense>
    }
}

#[server(HandPickSong)]
pub async fn pick_song(song_id: i32) -> Result<(), ServerFnError> {
    Setlist::set_songs(vec![song_id])
        .await
        .map_err(ServerFnError::from)
}

#[component]
pub fn SongView(
    song: Song,
    songs_resource: Resource<(), Result<Vec<Song>, ServerFnError>>,
    setlist_resource: Resource<(), Result<Setlist, ServerFnError>>,
) -> impl IntoView {
    let set_song_id = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a set_played signal provided");

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

    let pick_song_action = create_action(
        |input: &(i32, Resource<(), Result<Setlist, ServerFnError>>)| {
            let input = input.to_owned();
            async move {
                pick_song(input.0).await.map(|res| {
                    input.1.refetch();
                    res
                })
            }
        },
    );

    view! {
      <tr>
        <td>
          <div class="flex items-center gap-3">
            <div class="avatar">
              <div class="mask mask-squircle w-12 h-12">
                <AlbumArt base64_encoded_string=song.album_art/>
              </div>
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
                set_song_id.update(|id| *id = Some(song.id));
            }
          >

            <i class="fa fa-play"></i>
          </button>
        </td>
        <td>
            <div class="join">
          <button
            type="button"
            class="btn btn-outline btn-error join-item"
            on:click=move |_| {
                set_played_action.dispatch((song.id, songs_resource));
            }
          >

            <i class="fa-solid fa-check"></i>
          </button>
          <button
            type="button"
            class="btn btn-outline btn-error join-item"
            on:click=move |_| {
                pick_song_action.dispatch((song.id, setlist_resource));
            }
          >

            <i class="fa-solid fa-bookmark"></i>
          </button>
        </div>
        </td>
      </tr>
    }
}
