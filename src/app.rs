use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{gig::Gig, home::HomePage};
use crate::components::gigs::Gigs;
use crate::components::song_text::SongText;
use crate::error_template::{AppError, ErrorTemplate};

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
      <div class="min-h-screen bg-ctp-base text-ctp-text">
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
          <nav class="bg-ctp-surface0 shadow-md z-100">
            <div class="mx-auto max-w px-4 py-2">
              <div class="flex flex-1 items-center justify-center items-stretch justify-start">
                <div class="flex flex-1 items-center">
                  <a href="/" class="text-ctp-text rounded-md text-2xl font-bold">
                    <img src="/android-chrome-192x192.png" width=60 height=60/>
                  </a>
                </div>
                <div class="ml-6 block items-center">
                  <div class="flex space-x-4">
                    <a
                      href="/gigs"
                      class="bg-ctp-overlay2 text-ctp-mantle rounded-md px-3 py-2 text-md font-medium"
                    >
                      Gigs
                    </a>
                  </div>
                </div>
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
      </div>
    }
}
