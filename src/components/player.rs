use leptos::*;

use crate::components::shared::AlbumArt;
use crate::models::song::Song;

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

    view! {
      <div class="flex flex-col items-center justify-center sticky top-0 z-10 mx-2">
        <Transition fallback=|| {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"Loading song"</div> }
        }>
          {move || {
              match song_resource.get() {
                  Some(Ok(Some(song))) => {
                      view! {
                        <div class="flex-1 flex-col w-full rounded-md shadow-lg pb-2 bg-ctp-surface1 mt-1 p-1">
                          <SelectedSongView song=song.clone()/>
                          <AudioPlayer id=song.id source=song.gs_url/>
                        </div>
                      }
                          .into_view()
                  }
                  _ => view! {}.into_view(),
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
          <AlbumArt title=song.sanitized_title.clone() width=80 height=80/>
        </figure>
        <div class="flex-1 flex-cols">
          <div class="text-lg font-bold">{song.title}</div>
          <div>{song.artist}</div>
        </div>
      </div>
    }
}

#[component]
fn AudioPlayer(id: i32, source: Option<String>) -> impl IntoView {
    // let set_song_id =
    //     use_context::<WriteSignal<Option<i32>>>().expect("set_song_id context expected");

    // let get_setlist = use_context::<ReadSignal<Vec<i32>>>().expect("get_setlist context expected");
    //
    // let next_id = if let Some(current_index) = get_setlist.get().iter().position(|&x| x == id) {
    //     *get_setlist
    //         .get()
    //         .iter()
    //         .cycle()
    //         .nth(current_index + 1)
    //         .unwrap()
    // } else {
    //     id
    // };

    view! {
      <div class="flex mt-1">
        {if let Some(source) = source {
            view! {
              <audio class="grow" controls autoplay
                // on:ended=move |_| {set_song_id.update(|id| *id = Some(next_id))}
                  >
                <source src=source/>
              </audio>
            }
                .into_view()
        } else {
            view! { <div class="rounded-lg shadow-lg px-2 py-1">"No audio file available"</div> }
                .into_view()
        }}

      </div>
    }
}
