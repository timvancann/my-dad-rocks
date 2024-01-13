use base64::prelude::*;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use leptos::*;
use std::io::Read;

use crate::models::song::Song;

#[cfg(feature = "ssr")]
fn read_mp3_from_disk(path: String) -> Result<String, ServerFnError> {
    let file_path = format!("./assets/{}", path);
    let data = std::fs::read(file_path).unwrap();
    let encoded = STANDARD.encode(&data);
    return Ok(encoded);
}

#[server(GetMp3)]
pub async fn get_mp3(song_id: Option<i32>) -> Result<String, ServerFnError> {
    match song_id {
        Some(id) => Song::get(id)
            .await
            .map(|s| s.audio_file_path)
            .map(read_mp3_from_disk)?,
        None => Err(ServerFnError::MissingArg("Missing song id".to_string())),
    }
}

#[component]
pub fn Player(song_id: ReadSignal<Option<i32>>) -> impl IntoView {
    let song = create_resource(move || song_id.get(), get_mp3);

    fn create_data_uri_from(base64_encoded_string: String) -> String {
        format!("data:audio/mpeg;base64,{}", base64_encoded_string)
    }

    view! {
      <div class="text-center">
        <Suspense fallback=|| {
            view! { <div>"Loading song"</div> }
        }>
          {move || match song.get() {
              Some(maybe_path) => {
                  match maybe_path {
                      Ok(path) => {
                          view! {
                            <div>
                              <audio
                                controls
                                class="embed-responsive-item col-sm-12"
                                src=create_data_uri_from(path)
                              ></audio>
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
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let file_path = "./assets/./mp3/alive.mp3";
        let data = std::fs::read(file_path).unwrap();
        let encoded = STANDARD.encode(&data);
        let data_uri = format!("data:audio/mpeg;base64,{}", encoded);
        assert!(false)
    }
}
