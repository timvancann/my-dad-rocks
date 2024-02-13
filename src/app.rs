use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::gigs::Gigs;
use crate::components::song_text::SongText;
use crate::components::{gig::Gig, home::HomePage};

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
      <Router>
        <nav>
          <div class="navbar z-20 bg-base-200 shadow-lg">
            <div class="flex-1">
              <a class="btn btn-ghost text-xl" href="/">
                MDR
              </a>
            </div>
            <div class="flex-none">
              <ul class="menu menu-horizontal px-1">
                <li>
                  <a class="font-bold" href="/gigs">
                    Gigs
                  </a>
                </li>
              </ul>
            </div>
          </div>
        </nav>
        <div class="container mx-auto">
          <main>
            <Routes>
              <Route path="/" view=HomePage/>
              <Route path="/gigs" view=Gigs/>
              <Route path="/gig/:id" view=Gig/>
              <Route path="/lyric/:id" view=SongText/>
            </Routes>
          </main>
        </div>
      </Router>
    }
}
