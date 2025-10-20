fn is_id_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_')
}

/// gets the channel id from it's url
pub fn from_channel_url(identifier: &str) -> Result<String, String> {
    // if it doesnt look like a url, just assume it is an identifier
    if identifier.chars().all(is_id_char) {
        return Ok(identifier.to_string());
    }

    let index = if let Some(index) = identifier.find("/channel/") {
        index + 9
    } else {
        return Err(format!("Cannot find channel id from string `{identifier}`"));
    };

    Ok(identifier[index..]
        .chars()
        .take_while(|c| is_id_char(*c))
        .collect())
}

pub fn from_video_url(identifier: &str) -> Result<String, String> {
    if identifier.chars().all(is_id_char) {
        return Ok(identifier.to_string());
    }

    let index = if let Some(index) = identifier.find("?v=") {
        index + 3
        // and also "youtu.be/"
    } else if let Some(index) = identifier.find("youtu.be/") {
        index + 9
    } else {
        return Err(format!("Cannot find video id from string `{identifier}`"));
    };

    Ok(identifier[index..]
        .chars()
        .take_while(|c| is_id_char(*c))
        .collect())
}

pub fn from_playlist_url(identifier: &str) -> Result<String, String> {
    if identifier.chars().all(is_id_char) {
        return Ok(identifier.to_string());
    }

    // the id can come after "list=" in urls
    let index = if let Some(index) = identifier.find("list=") {
        if identifier.len() < index + 39 {
            return Err(format!(
                "Cannot find plalyist id from string `{identifier}`"
            ));
        }
        index + 5
    } else {
        return Err(format!(
            "Cannot find playlist id from string `{identifier}`"
        ));
    };

    Ok(identifier[index..]
        .chars()
        .take_while(|c| is_id_char(*c))
        .collect())
}
