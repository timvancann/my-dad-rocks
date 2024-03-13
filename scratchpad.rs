let set_item_toggled = create_server_action::<SetItemToggled>();
let items_resource = create_resource(move || (set_item_toggled.version().get()), |_| get_items());


<Suspense fallback=move || {
        view! { <p>"Loading..."</p> }
    }>

      {
        let loaded_items: Memo<Vec<Item>> = create_memo(move |_| {
          items_resource.get()
            .unwrap_or(Ok(vec![]))
            .unwrap_or_default()
        });
        view!{
          <For
          {move || items_resource.track()}
            each=move || loaded_items.get().into_iter().enumerate()
            key=|(_, state)| state.id
            children=move |(index, _)| {
                let item = create_memo(move |_| {
                    loaded_items.with(|data| { data.get(index).unwrap().clone() })
                });
                move || view! { <ItemView 
                    item=item.get() 
                        index
                    loaded_items=loaded_items.get() 
                    pick_item set_item_played/> 
                }.into_view()
            }
          />
        }
      }
</Suspense>

#[component]
pub fn ItemView(
    item: Item,
    set_item_toggled: Action<SetItemToggled, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
          <Item item=item.clone()/>
          <button
            type="button"
            on:click=move |_| { 
                set_item_toggled.dispatch(SetItemToggled { item_id: item.id }) 
            }
          >
          Toggle
          </button>
    }
}
