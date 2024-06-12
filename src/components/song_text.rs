use leptos::*;
use leptos_router::*;

use crate::components::shared::get_song;
use crate::models::song::Song;

#[derive(Params, PartialEq)]
struct LyricParams {
    id: Option<usize>,
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

    let song_resource = create_resource(id, get_song);

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
                    </div>
                    {move || match edit_mode.get() {
                        true => view! {}.into_view(),
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
