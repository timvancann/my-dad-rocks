use leptos::*;

#[component]
pub fn AlbumArt(base64_encoded_string: String) -> impl IntoView {
    let image_uri = format!("data:image/png;base64,{}", base64_encoded_string);
    view! {
      <div>
        <img src=image_uri alt="Album art"/>
      </div>
    }
}
