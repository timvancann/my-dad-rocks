use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::song_item::SongItem;
use crate::models::setlist::Setlist;
use crate::{error_template::ErrorTemplate, models::song::Song};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HomePageSongs {
    setlist: Vec<Song>,
    all_songs: Vec<Song>,
}

#[server(GetSongs)]
pub async fn get_songs() -> Result<HomePageSongs, ServerFnError> {
    match Song::get_all().await {
        Ok(s) => Ok(HomePageSongs {
            setlist: s.clone().into_iter().filter(|s| s.should_play).collect(),
            all_songs: s,
        }),
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

type Act<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn Songs() -> impl IntoView {
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

    view! {
      <div class="grid grid-cols-8 gap-2 mb-4">
        <div class="font-bold text-2xl col-span-3 ml-3">Setlist</div>
        <div class="join col-span-4">
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

      <HomePageSongs
        songs_resource
        pick_song
        set_song_played
        picker=|hpr: HomePageSongs| { hpr.setlist }
      />

      <div class="divider"></div>
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Alle nummers</div>
      </div>

      <HomePageSongs
        songs_resource
        pick_song
        set_song_played
        picker=|hpr: HomePageSongs| { hpr.all_songs }
      />
    }
}

#[component]
pub fn HomePageSongs(
    songs_resource: Resource<(usize, usize, usize, usize), Result<HomePageSongs, ServerFnError>>,
    pick_song: Act<HandPickSong>,
    set_song_played: Act<SetSongPlayed>,
    picker: fn(HomePageSongs) -> Vec<Song>,
) -> impl IntoView {
    view! {
      <Transition fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
          {move || {
              if let Some(Ok(songs)) = songs_resource() {
                  view! { <SongListView songs=picker(songs) pick_song set_song_played/> }
                      .into_view()
              } else {
                  view! { <pre class="error">"Server Error"</pre> }.into_view()
              }
          }}

        </ErrorBoundary>
      </Transition>
    }
}

#[component]
pub fn SongListView(
    songs: Vec<Song>,
    pick_song: Action<HandPickSong, Result<(), ServerFnError>>,
    set_song_played: Action<SetSongPlayed, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
      <Suspense fallback=move || view! { <div></div> }>
        <div class="grid grid-cols-8 gap-2">
          {songs
              .clone()
              .into_iter()
              .map(move |song| view! { <SongView song pick_song set_song_played/> })
              .collect_view()}
        </div>
      </Suspense>
    }
}

#[component]
pub fn SongView(
    song: Song,
    pick_song: Action<HandPickSong, Result<(), ServerFnError>>,
    set_song_played: Action<SetSongPlayed, Result<(), ServerFnError>>,
) -> impl IntoView {
    let set_song_id = use_context::<WriteSignal<Option<i32>>>()
        .expect("Expected to have a set_played signal provided");

    let bookmark_class = match song.should_play {
        true => "btn btn-outline btn-error btn-disabled join-item",
        false => "btn btn-outline btn-error join-item",
    };

    view! {
      <div class="col-span-5 ml-4">
        <SongItem song=song.clone()/>
      </div>
      <div class="col-span-1">
        <button
          type="button"
          class="btn btn-primary btn-circle"
          on:click=move |_| {
              set_song_id.update(|id| *id = Some(song.id));
          }
        >

          <i class="fa fa-play"></i>
        </button>
      </div>
      <div class="col-span-2">
        <div class="join">
          <button
            type="button"
            class="btn btn-outline btn-error join-item"
            on:click=move |_| { set_song_played.dispatch(SetSongPlayed { song_id: song.id }) }
          >

            <i class="fa-solid fa-calendar-day"></i>
          </button>
          <button
            type="button"
            class=move || bookmark_class
            on:click=move |_| { pick_song.dispatch(HandPickSong { song_id: song.id }) }
          >

            <i class="fa-solid fa-bookmark"></i>
          </button>
        </div>
      </div>
    }
}
