use leptos::*;

use crate::models::gig::{Gig, GigModel};

#[server(GetGigs)]
pub async fn get_gigs() -> Result<Vec<GigModel>, ServerFnError> {
    Gig::get_all().await.map_err(ServerFnError::from)
}

#[server(CreateGig)]
pub async fn create_gig() -> Result<(), ServerFnError> {
    Gig::create().await.map_err(ServerFnError::from)
}

#[component]
pub fn Gigs() -> impl IntoView {
    let create_gig = create_server_action::<CreateGig>();
    let gigs_resource = create_resource(move || create_gig.version().get(), |_| get_gigs());

    view! {
      <div class="pt-4 pl-4">
        <button
          type="button"
          class="btn btn-accent"
          on:click=move |_| { create_gig.dispatch(CreateGig {}) }
        >
          <i class="fa-solid fa-plus"></i>
          Nieuw optreden
        </button>
        <ul class="timeline timeline-vertical">
          <Suspense fallback=move || {
              view! { <p>"Loading..."</p> }
          }>
            {move || {
                if let Some(Ok(gigs)) = gigs_resource() {
                    gigs.into_iter()
                        .map(move |gig| { view! { <Timeline gig/> }.into_view() })
                        .collect_view()
                } else {
                    view! { <div>"Loading..."</div> }.into_view()
                }
            }}

          </Suspense>
        </ul>
      </div>
    }
    .into_view()
}

#[component]
pub fn Timeline(gig: GigModel) -> impl IntoView {
    view! {
      <li>
        <hr/>
        <div class="timeline-start">
          <a href=format!("/gig/{}", gig.id)>{gig.date.to_string()}</a>
        </div>
        <div class="timeline-middle">
          <a href=format!("/gig/{}", gig.id)>
            <i class="fas fa-circle"></i>
          </a>
        </div>
        <div class="timeline-end timeline-box">
          <a href=format!("/gig/{}", gig.id)>{gig.venue}</a>
        </div>
        <hr/>
      </li>
    }
    .into_view()
}
