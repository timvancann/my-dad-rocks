use crate::{
    components::all_songs::AllSongs,
    components::{player::Player, random_selection::RandomSongView, setlist::SetlistView},
};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Stylesheet id="leptos" href="/pkg/my-dad-rocks.css"/>
      <Stylesheet
        id="font-awesome"
        href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css"
      />
      <Title text="My Dad Rocks"/>

      // content for this welcome page
      <div class="container mx-auto">
        <main>
          <HomePage/>
        </main>
      </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (get_song_id, set_song_id) = create_signal(None::<i32>);
    provide_context(set_song_id);
    provide_context(get_song_id);

    view! {
      <div>
        <Intro/>
        <div class="divider"></div>
        <Player/>
        <RandomSongView/>
        <div class="divider"></div>
        // <SetlistView/>
        <div class="divider"></div>
        <AllSongs/>
      </div>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
      <div class="flex justify-center">
        <div class="font-bold text-3xl">My Dad Rocks</div>
      </div>
    }
}
