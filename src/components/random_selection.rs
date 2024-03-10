use leptos::*;

use crate::models::song::Song;

#[server(GetRandomSong, "/api", "GetJson")]
pub async fn get_random_song() -> Result<Song, ServerFnError> {
    use rand::distributions::{Distribution, WeightedIndex};
    let mut songs = Song::get_all().await?;

    songs.sort_by(|a, b| a.last_played_at.cmp(&b.last_played_at));
    let weights: Vec<_> = songs
        .iter()
        .enumerate()
        .map(|(i, _)| (songs.len() - i) as u32)
        .collect();
    let dist = WeightedIndex::new(weights)?;
    let mut rng = rand::thread_rng();
    Ok(songs[dist.sample(&mut rng)].clone())
}

#[server(GetSong, "/api", "GetJson")]
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
pub fn RandomSongView() -> impl IntoView {
    let set_song_id = use_context::<WriteSignal<Option<i32>>>().expect("set_song context expected");

    let song_action = create_action(move |_: &()| async move {
        let song = get_random_song().await;
        match song {
            Ok(s) => set_song_id.update(|id| *id = Some(s.id)),
            Err(_) => set_song_id.update(|id| *id = None),
        }
    });

    view! {
      <div class="flex justify-center mt-4">
        <button
          class="border-0 rounded-full px-3 py-2 shadow-lg bg-ctp-green text-ctp-mantle"
          on:click=move |_| { song_action.dispatch(()) }
        >
          <span class="fa-solid fa-shuffle"></span>
          Willekeurig
        </button>
      </div>
    }
}
