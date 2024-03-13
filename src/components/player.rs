use leptos::*;

use crate::components::albumart::AlbumArt;
use crate::models::song::{Song, ThumbnailType};

#[server(GetPlayerThumbnail, "/api", "GetJson")]
pub async fn get_player_thumbnail(song_id: Option<i32>) -> Result<Option<String>, ServerFnError> {
    match song_id {
        Some(id) => {
            let song = Song::get(id).await?;
            let thumbnail =
                Song::get_picture_as_base64(song.audio_file_path, ThumbnailType::Player);
            Ok(Some(thumbnail))
        }
        None => Ok(None),
    }
}

#[server(GetSong, "/api", "GetJson")]
pub async fn get_song(song_id: Option<i32>) -> Result<Option<Song>, ServerFnError> {
    match song_id {
        Some(id) => Ok(Some(Song::get(id).await?)),
        None => Ok(None),
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let get_song_id =
        use_context::<ReadSignal<Option<i32>>>().expect("get_song_id context expected");

    let song_resource = create_resource(move || get_song_id.get(), get_song);
    let thumbnail_resource = create_resource(move || get_song_id.get(), get_player_thumbnail);

    view! {
      <div class="flex flex-col items-center justify-center sticky top-0 z-10 mx-2">
        <Transition fallback=|| {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"Loading song"</div> }
        }>
          {move || {
            match (song_resource.get(), thumbnail_resource.get()) {
              (Some(Ok(Some(song))), Some(Ok(Some(thumbnail)))) => {
                view! {
                  <div class="flex-1 flex-col w-full rounded-md shadow-lg pb-2 bg-ctp-surface1 mt-1 p-1">
                    <SelectedSongView song=song.clone() thumbnail/>
                    <AudioPlayer source=song.gs_url/>
                  </div>
                }
                    .into_view()
              },
              _ => view! {}.into_view()
          }}}

        </Transition>
      </div>
    }
}

#[component]
fn SelectedSongView(song: Song, thumbnail: String) -> impl IntoView {
    view! {
      <div class="flex grow">
        <figure class="flex w-20 h-20 mr-4">
          <AlbumArt base64_encoded_string=thumbnail/>
        </figure>
        <div class="flex-1 flex-cols">
          <div class="text-lg font-bold">{song.title}</div>
          <div>{song.artist}</div>
        </div>
      </div>
    }
}

#[component]
fn AudioPlayer(source: Option<String>) -> impl IntoView {
    view! {
      <div class="flex mt-1">
      {if let Some(source) = source {
          view! {
            <audio class="grow" controls autoplay>
            <source src=source/>
            </audio>
          }.into_view()
        } else {
          view! {
            <div class="rounded-lg shadow-lg px-2 py-1">"No audio file available"</div>
        }.into_view()
                
        }
    }
      </div>
    }
}
