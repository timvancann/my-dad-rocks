use leptos::*;

#[component]
pub fn Horizontal() -> impl IntoView {
    view! { <div class="h-px m-3 mt-6 bg-ctp-surface1 border-0"></div> }.into_view()
}

#[component]
pub fn AlbumArt(title: String, width: u32, height: u32) -> impl IntoView {
    let mut art = title;
    art.push_str(".png");
    let path = format!("/album_art/{}", art);

    view! { <img src=path width=width height=height/> }
}
