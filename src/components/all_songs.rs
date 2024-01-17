use leptos::*;

use crate::{
    components::song::SongListView,
    models::song::Song,
};

#[server(GetSongs)]
pub async fn get_songs() -> Result<Vec<Song>, ServerFnError> {
    match Song::get_all().await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[component]
pub fn AllSongs() -> impl IntoView {
    let all_songs = create_resource(|| (), |_| async move { get_songs().await });
    provide_context(all_songs);

    view! {
      <div class="flex items-center justify-between mb-4 ml-4 mr-4">
        <div class="font-bold text-2xl">Alle nummers</div>
      </div>
      <SongListView/>
    }
}
