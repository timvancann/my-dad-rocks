use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::components::shared::get_song;
use crate::models::song::Song;

#[server(EditSong)]
pub async fn edit_song(song: EditSongData) -> Result<(), ServerFnError> {
    let res = Song::update(song).await?;
    leptos_axum::redirect("/");
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct EditSongData {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub bpm: i32,
    pub lyrics: String,
}

#[derive(Params, PartialEq)]
struct SongParams {
    id: Option<usize>,
}

#[component]
pub fn EditSongView() -> impl IntoView {
    let params = use_params::<SongParams>();
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
      <Suspense>

        {if let Some(Ok(song)) = song_resource.get() {
            view! { <EditSongForm song/> }.into_view()
        } else {
            view! { <div>"Tits"</div> }.into_view()
        }}

      </Suspense>
    }
}

#[component]
pub fn EditSongForm(song: Song) -> impl IntoView {
    let action = create_server_action::<EditSong>();
    view! {
      <ActionForm action=action class="mx-2 mt-2">
        <button type="submit" class="border-0 rounded-md px-3 py-2 shadow-lg bg-ctp-lavender text-ctp-mantle">Opslaan</button>

        <input type="hidden" name="song[id]" value={song.id}/>

        <div class="grid md:grid-cols-2 md:gap-6 mt-6">
          <Input title="Artiest".to_string() value=song.artist entity="song[artist]".to_string()/>
          <Input title="Titel".to_string() value=song.title entity="song[title]".to_string()/>
        </div>
        <InputNumber title="BPM".to_string() value=song.bpm.unwrap() entity="song[bpm]".to_string()/>
        <textarea
          type="text"
          class="textarea textarea-bordered w-full max-w p-2 h-screen white-space:pre;"
          placeholder="Edit lyrics"
          name="song[lyrics]"
          value={song.lyrics.to_string()}
        >
        {song.lyrics}
        </textarea>
      </ActionForm>
    }
}

#[component]
pub fn Input(title: String, value: String, entity: String) -> impl IntoView {
    view! {
      <div class="relative z-0 w-full mb-5 group">
        <input
          type="text"
          name=entity.clone()
          id=entity.clone()
          class="block py-2.5 px-0 w-full text-sm bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 peer"
          value=value
        />
        <label
          for=entity.clone()
          class="peer-focus:font-medium absolute text-sm duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
        >
          {title}
        </label>
      </div>
    }
}

#[component]
pub fn InputNumber(title: String, value: i32, entity: String) -> impl IntoView {
    view! {
      <div class="relative z-0 w-full mb-5 group">
        <input
          type="number"
          name=entity.clone()
          id=entity.clone()
          class="block py-2.5 px-0 w-full text-sm bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 peer"
          value={value}
        />
        <label
          for=entity.clone()
          class="peer-focus:font-medium absolute text-sm duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
        >
          {title}
        </label>
      </div>
    }
}

#[component]
fn ViewLyric(song: Song) -> impl IntoView {
    view! {
      <div class="white-space:pre; mt-2">
        {song
            .lyrics
            .split('\n')
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
