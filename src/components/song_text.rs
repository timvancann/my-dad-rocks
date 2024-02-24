use leptos::*;
use leptos_router::*;

use crate::models::song::Song;

#[derive(Params, PartialEq)]
struct LyricParams {
    id: Option<usize>,
}

#[server(GetSong)]
pub async fn get_song(id: usize) -> Result<Song, ServerFnError> {
    Song::get(id as i32).await.map_err(ServerFnError::from)
}

#[component]
pub fn SongText() -> impl IntoView {
    let params = use_params::<LyricParams>();

    let edit_mode = create_rw_signal(false);
    provide_context(edit_mode);

    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.unwrap_or_default())
                .unwrap_or_default()
        })
    };

    let update_lyrics = create_server_action::<UpdateLyrics>();
    let song_resource = create_resource(move || id(), |args| get_song(args));

    view! {
      <div class="m-2">
        <Suspense fallback=|| {
            view! {}.into_view()
        }>
          {move || {
              if let Some(Ok(song)) = song_resource.get() {
                  view! {
                    <div class="flex items-center justify-between">
                      <div>
                        <div class="text-xl font-bold text-nowrap">{song.title.to_string()}</div>
                      </div>
                      <div class="flex">
                        <EditButton song_resource/>
                      </div>
                    </div>
                    {move || match edit_mode.get() {
                        true => view! { <EditLyric song=song.clone() update_lyrics/> }.into_view(),
                        false => view! { <ViewLyric song=song.clone()/> }.into_view(),
                    }}
                  }
                      .into_view()
              } else {
                  view! {}.into_view()
              }
          }}

        </Suspense>
      </div>
    }
}

#[component]
fn EditButton(song_resource: Resource<usize, Result<Song, ServerFnError>>) -> impl IntoView {
    let edit_mode =
        use_context::<RwSignal<bool>>().expect("EditButton must be used within a context");

    let edit_mode_active_class = move || match edit_mode.get() {
        true => "rounded-full border-2 p-3",
        false => "rounded-full border-2 p-3 bg-ctp-mauve text-ctp-crust",
    };

    view! {
      <button
        type="button"
        class=move || edit_mode_active_class()
        on:click=move |_| {
            edit_mode.set(!edit_mode.get());
            if !edit_mode.get() {
                song_resource.refetch()
            }
        }
      >

        <i class="fa-solid fa-edit"></i>
      </button>
    }
}

#[server(UpdateLyrics)]
pub async fn update_lyrics(id: i32, lyrics: String) -> Result<(), ServerFnError> {
    Song::update_lyrics(id, lyrics)
        .await
        .map_err(ServerFnError::from)
}

#[component]
fn EditLyric(
    song: Song,
    update_lyrics: Action<UpdateLyrics, Result<(), ServerFnError>>,
) -> impl IntoView {
    let (lyrics, set_lyrics) = create_signal(song.lyrics);

    view! {
      <div class="mt-2">
        <textarea
          type="text"
          class="textarea textarea-bordered w-full max-w p-2 h-screen white-space:pre;"
          placeholder="Edit lyrics"
          on:input=move |e| {
              update_lyrics
                  .dispatch(UpdateLyrics {
                      id: song.id,
                      lyrics: event_target_value(&e),
                  })
          }
        >

          {lyrics}
        </textarea>
      </div>
    }
}

#[component]
fn ViewLyric(song: Song) -> impl IntoView {
    view! {
      <div class="white-space:pre; mt-2">
        {song
            .lyrics
            .split("\n")
            .into_iter()
            .map(|line| {
                if line.is_empty() {
                    view! { <br/> }.into_view()
                } else {
                    view! { <p class="m-1">{line.to_string()}</p> }.into_view()
                }
            })
            .collect_view()}
      </div>
    }
}
