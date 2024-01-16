use crate::{
    components::all_songs::AllSongs,
    components::{
        footer::Footer, player::Player, random_selection::RandomSongView, setlist::SetlistView,
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
          <main>
            <Routes>
              <Route path="" view=HomePage/>
            </Routes>
          </main>
        </Router>
      </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (song_id, set_song_id) = create_signal(None::<i32>);

    view! {
      <div>
        <Intro/>
        <div class="divider"></div>
        <Player song_id/>
        <RandomSongView song_id set_song_id/>
        <div class="divider"></div>
        <SetlistView set_song_id/>
        <div class="divider"></div>
        <AllSongs set_song_id/>
        <Footer/>
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
