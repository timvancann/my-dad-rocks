use base64::prelude::*;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::components::albumart::AlbumArt;
use crate::models::song::Song;

#[cfg(feature = "ssr")]
pub fn read_mp3_from_disk(path: &String) -> Result<String, ServerFnError> {
    let file_path = format!("./assets/{}", path);
    let data = std::fs::read(file_path).unwrap();
    let encoded = STANDARD.encode(&data);
    return Ok(encoded);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFile {
    mp3: String,
    song: Song,
}

#[server(GetMp3)]
pub async fn get_mp3(song_id: Option<i32>) -> Result<AudioFile, ServerFnError> {
    match song_id {
        Some(id) => {
            let song = Song::get(id).await?;
            let mp3 = read_mp3_from_disk(&song.audio_file_path)?;
            return Ok(AudioFile { mp3, song });
        }
        None => Err(ServerFnError::MissingArg("Missing song id".to_string())),
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let get_song_id =
        use_context::<ReadSignal<Option<i32>>>().expect("get_song_id context expected");

    let song = create_resource(move || get_song_id.get(), get_mp3);

    fn create_data_uri_from(base64_encoded_string: String) -> String {
        format!("data:audio/mpeg;base64,{}", base64_encoded_string)
    }

    view! {
      <div class="flex flex-col items-center justify-center">
        <div class="text-center">
          <Suspense fallback=|| {
              view! { <div>"Loading song"</div> }
          }>
            {move || match song.get() {
                Some(maybe_audio_file) => {
                    match maybe_audio_file {
                        Ok(audio_file) => {
                            let data_uri = create_data_uri_from(audio_file.mp3);
                            let song = audio_file.song;
                            view! {
                              <div class="grid grid-cols-1">
                                <SelectedSongView song/>
                                <AudioPlayer data_uri/>
                              </div>
                            }
                        }
                        _ => view! { <div>"No audio file selected"</div> },
                    }
                }
                None => view! { <div>"No audio file selected"</div> },
            }}

          </Suspense>
        </div>
      </div>
    }
}

#[component]
fn SelectedSongView(song: Song) -> impl IntoView {
    let song_filepath = song.audio_file_path;
    view! {
        <div class="card card-side bg-slate-100 shadow-xl">
        <figure class="max-w-52">
          <AlbumArt song_filepath/>
        </figure>
        <div class="card-body min-w-72">
          <p class="text-xl font-medium">{song.title}</p>
          <p>{song.artist}</p>
        </div>
      </div>
    }
}

#[component]
fn AudioPlayer(data_uri: String) -> impl IntoView {
    view! {
      <div class="flex">
        <audio class="grow" controls src=data_uri></audio>
      </div>
    }
}
