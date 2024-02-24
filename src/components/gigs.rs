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
          class="border-0 rounded-full px-3 py-2 shadow-md bg-ctp-teal text-ctp-mantle"
          on:click=move |_| { create_gig.dispatch(CreateGig {}) }
        >
          <i class="fa-solid fa-plus"></i>
          Nieuw optreden
        </button>
        <ol class="relative border-s border-gray-200 mx-2 mt-4">
          <Transition>
            {move || {
                gigs_resource
                    .get()
                    .unwrap_or_else(|| Ok(vec![]))
                    .unwrap_or_default()
                    .into_iter()
                    .map(move |gig| { view! { <Timeline gig/> }.into_view() })
                    .collect_view()
            }}

          </Transition>
        </ol>

      </div>
    }
    .into_view()
}

#[component]
pub fn Timeline(gig: GigModel) -> impl IntoView {
    view! {
      <li class="mb-10 ms-6">

        <a href=format!("/gig/{}", gig.id)>
          <span class="absolute flex items-center justify-center w-4 h-4 mt-4 rounded-full -start-2 ring-8 bg-ctp-surface1 ring-ctp-surface1">
            <i class="fas fa-calendar-day"></i>
          </span>
          <h3 class="flex items-center mb-1 text-lg font-semibold text-ctp-text">{gig.venue}</h3>
          <time class="block mb-2 text-sm font-normal leading-none text-ctp-overlay0">
            {gig.date.to_string()}
          </time>
        // <p class="mb-4 text-base font-normal text-ctp-subtext1">Besloten feest</p>
        </a>
      </li>
    }
    .into_view()
}
