use crate::{
    components::all_songs::AllSongs,
    components::{gigs::Gig, player::Player, random_selection::RandomSongView},
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
        <Router>
          <nav>
            <div class="navbar bg-neutral z-20">
              <div class="navbar-start">
                <a class="btn btn-ghost text-xl" href="/">My Dad Rocks!</a>
              </div>
              <div class="navbar-center flex">
                <ul class="menu menu-horizontal px-1">
                  <li>
                    <a href="/gig/1">Breda 2024</a>
                  </li>
                </ul>
              </div>
            </div>
          </nav>
          <main>
            <Routes>
              <Route path="/" view=HomePage/>
              <Route path="/gig/:id" view=Gig/>
            </Routes>

          </main>
        </Router>
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
        <Player/>
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
