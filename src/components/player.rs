use base64::prelude::*;
use base64::engine::general_purpose::STANDARD;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::albumart::AlbumArt;
use crate::models::song::Song;

#[cfg(feature = "ssr")]
pub fn read_mp3_from_disk(path: &String) -> Result<String, ServerFnError> {
    let file_path = format!("./assets/{}", path);
    let data = std::fs::read(file_path).unwrap();
    let encoded = STANDARD.encode(data);
    Ok(encoded)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFile {
    mp3: String,
    song: Song,
}

#[server(GetMp3)]
pub async fn get_mp3(song_id: Option<i32>) -> Result<Option<AudioFile>, ServerFnError> {
    match song_id {
        Some(id) => {
            let song = Song::get(id).await?;
            let mp3 = read_mp3_from_disk(&song.audio_file_path)?;
            Ok(Some(AudioFile { mp3, song }) )
        }
        None => Ok(None)
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let get_song_id = use_context::<ReadSignal<Option<i32>>>().expect("get_song_id context expected");

    let audio_file_resource = create_resource(move || get_song_id.get(), get_mp3);

    fn create_data_uri_from(base64_encoded_string: String) -> String {
        format!("data:audio/mpeg;base64,{}", base64_encoded_string)
    }

    view! {
      <div class="flex flex-col items-center justify-center sticky top-0 z-10 mx-2">
        <Transition fallback=|| {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"Loading song"</div> }
        }>
          {move || {
              if let Some(Ok(Some(audio_file))) = audio_file_resource.get() {
                  let data_uri = create_data_uri_from(audio_file.mp3);
                  let song = audio_file.song;
                  view! {
                    <div class="flex-1 flex-col w-screen rounded-b-lg shadow-lg">
                      <SelectedSongView song/>
                      <AudioPlayer data_uri/>
                    </div>
                  }
                      .into_view()
              } else {
                  view! {}.into_view()
              }
          }}

        </Transition>
      </div>
    }
}

#[component]
fn SelectedSongView(song: Song) -> impl IntoView {
    view! {
      <div class="flex grow">
        <figure class="flex w-20 h-20 mr-4">
          <AlbumArt base64_encoded_string=song.album_art/>
        </figure>
        <div class="flex-1 flex-cols">
          <div class="text-lg font-bold">{song.title}</div>
          <div>{song.artist}</div>
        </div>
      </div>
    }
}

#[component]
fn AudioPlayer(data_uri: String) -> impl IntoView {
    view! {
      <div class="flex">
        <audio class="grow" controls autoplay src=data_uri></audio>
      </div>
    }
}
