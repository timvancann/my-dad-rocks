use crate::{
    components::all_songs::AllSongs,
    components::{player::Player, random_selection::RandomSongView, setlist::SetlistView},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      <Stylesheet id="leptos" href="/pkg/my-dad-rocks-ssr.css"/>
      <Stylesheet
        id="boostrap"
        href="https://cdn.jsdelivr.net/npm/bootstrap@4.3.1/dist/css/bootstrap.min.css"
      />
      <Stylesheet
        id="fa"
        href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
      />
      <Stylesheet
        id="boostrap-icons"
        href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css"
      />
      <script src="https://code.jquery.com/jquery-3.3.1.slim.min.js"></script>
      <script src="https://cdn.jsdelivr.net/npm/popper.js@1.14.7/dist/umd/popper.min.js"></script>
      <script src="https://cdn.jsdelivr.net/npm/bootstrap@4.3.1/dist/js/bootstrap.min.js"></script>

      // sets the document title
      <Title text="My Dad Rocks"/>

      // content for this welcome page
      <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { <ErrorTemplate outside_errors/> }.into_view()
      }>
        <div class="container">
          <main>
            <Routes>
              <Route path="" view=HomePage/>
            </Routes>
          </main>
        </div>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (song_id, set_song_id) = create_signal(None::<i32>);

    view! {
      <div>
        <h1 class="text-center display-4 text-dark">My Dad Rocks</h1>
        <hr class="border border-secondary border-2 opacity-75"/>
        <Player song_id/>
        <hr class="border border-primary border-2 opacity-75"/>
        <RandomSongView song_id set_song_id/>
        <SetlistView set_song_id/>
        <hr class="border border-primary border-2 opacity-75"/>
        <AllSongs set_song_id/>
      </div>
    }
}
