use leptos::*;

#[component]
pub fn Promo() -> impl IntoView {
    view! {
      <div class="grid grid-flow-row auto-rows-max gap-2 mt-4">
        <PromoImage source="/promo/logo_pop.png" title="Pop"/>
        <PromoImage source="/promo/logo_round.png" title="Rond"/>
        <PromoImage source="/promo/logo_square.png" title="Vierkant"/>
        <PromoImage source="/promo/logo_transparent.png" title="Transparant"/>
      </div>
    }
}

#[component]
pub fn PromoImage(source: &'static str, title: &'static str) -> impl IntoView {
    view! {
      <div class="grid grid-cols-1 justify-items-center mt-2">
        <img src=source class="size-60"/>
        <div class="text">{title}</div>
      </div>
    }
}
