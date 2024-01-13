use leptos::*;

use crate::models::song::Song;

#[server(GetRandomSong)]
pub async fn get_random_song() -> Result<Song, ServerFnError> {
    use rand::distributions::{Distribution, WeightedIndex};
    let mut songs = Song::get_all().await?;

    songs.sort_by(|a, b| a.last_played_at.cmp(&b.last_played_at));
    let weights: Vec<_> = songs
        .iter()
        .enumerate()
        .map(|(i, _)| (songs.len() - i) as u32)
        .collect();
    let dist = WeightedIndex::new(&weights)?;
    let mut rng = rand::thread_rng();
    Ok(songs[dist.sample(&mut rng)].clone())
}

#[server(GetSong)]
pub async fn get_song(song_id: Option<i32>) -> Result<Song, ServerFnError> {
    match song_id {
        Some(id) => match Song::get(id).await {
            Ok(s) => Ok(s),
            Err(e) => Err(ServerFnError::from(e)),
        },
        None => Err(ServerFnError::MissingArg("Missing song id".to_string())),
    }
}

#[component]
pub fn RandomSongView(
    song_id: ReadSignal<Option<i32>>,
    set_song_id: WriteSignal<Option<i32>>,
) -> impl IntoView {
    let song_action = create_action(move |_: &()| async move {
        let song = get_random_song().await;
        match song {
            Ok(s) => set_song_id.update(|id| *id = Some(s.id)),
            Err(_) => set_song_id.update(|id| *id = None),
        }
    });

    let song_resource = create_resource(move || song_id.get(), get_song);

    view! {
      <h2 class="text-center display-7 text-dark">Willekeurige selectie</h2>
      <div class="row">
        <div class="col">
          <button class="btn btn-primary" on:click=move |_| { song_action.dispatch(()) }>
            <i class="bi bi-shuffle"></i>
            Willekeurig
          </button>
        </div>
        <div class="col">
          <Suspense fallback=|| {
              view! { <div>"Loading song"</div> }
          }>
            {move || match song_resource.get() {
                Some(maybe_song) => {
                    match maybe_song {
                        Ok(s) => {
                            view! {
                              <div>
                                <SongView song=s/>
                              </div>
                            }
                        }
                        _ => view! { <div>"No song selected"</div> },
                    }
                }
                None => view! { <div>"No song  selected"</div> },
            }}

          </Suspense>
        </div>
      </div>
    }
}

#[component]
fn SongView(song: Song) -> impl IntoView {
    view! {
      <div class="alert alert-info" role="alert">
        {song.title}
      </div>
    }
}
