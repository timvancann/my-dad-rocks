use crate::components::gigs::Gigs;
use crate::components::{gig::Gig, home::HomePage};
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
            <div class="navbar z-20">
              <div class="navbar-start">
                <a class="btn btn-ghost text-xl" href="/">
                  My Dad Rocks!
                </a>
              </div>
              <div class="navbar-center flex">
                <ul class="menu menu-horizontal px-1">
                  <li>
                    <a class="btn btn-ghost text-lg" href="/gigs">
                      Gigs
                    </a>
                  </li>
                </ul>
              </div>
            </div>
          </nav>
          <main>
            <Routes>
              <Route path="/" view=HomePage/>
              <Route path="/gigs" view=Gigs/>
              <Route path="/gig/:id" view=Gig/>
            </Routes>
          </main>
        </Router>
      </div>
    }
}
