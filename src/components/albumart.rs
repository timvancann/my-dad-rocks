use leptos::*;

#[server(GetPicture)]
pub async fn get_picture_as_base64(song_filepath: String) -> Result<String, leptos::ServerFnError> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use id3::{Tag, TagLike};
    let file_path = format!("./assets/{}", song_filepath);

    let tag = Tag::read_from_path(file_path)?;
    if let Some(pic) = tag.pictures().next() {
        let encoded = STANDARD.encode(&pic.data);
        return Ok(encoded);
    }

    Err(leptos::ServerFnError::MissingArg(
        "Missing song album".to_string(),
    ))
}

#[component()]
pub fn AlbumArt(song_filepath: String) -> impl IntoView {
    let picture_resource = create_resource(move || song_filepath.clone(), get_picture_as_base64);
    fn create_data_uri_from(base64_encoded_string: String) -> String {
        format!("data:image/jpeg;base64,{}", base64_encoded_string)
    }

    view! {
      <Suspense fallback=move || {
          view! { <div></div> }
      }>

        {match picture_resource.get() {
            Some(picture) => {
                match picture {
                    Ok(base64) => {
                        let image_uri = create_data_uri_from(base64);
                        view! {
                          <div>
                            <img src=image_uri alt="Avatar Tailwind CSS Component"/>
                          </div>
                        }
                    }
                    Err(_) => view! { <div>"Tits"</div> },
                }
            }
            None => view! { <div>"No album"</div> },
        }}

      </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
