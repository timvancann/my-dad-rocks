use leptos::*;

use crate::components::homepage_songs::Songs;
use crate::components::player::Player;

#[component]
pub fn HomePage() -> impl IntoView {
    let (get_song_id, set_song_id) = create_signal(None::<i32>);
    provide_context(get_song_id);
    provide_context(set_song_id);

    let (get_setlist, set_setlist) = create_signal(Vec::<i32>::new());
    provide_context(get_setlist);
    provide_context(set_setlist);

    view! {
      <div>
        <Player/>
        <div class="divider"></div>
        <Songs/>
      </div>
    }
}
