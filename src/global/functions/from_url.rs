/// gets the channel id from it's url
pub fn from_channel_url(identifier: &str) -> Result<String, String> {
    // a youtube channel id is exactly 24 characters long, so if the identifier to 24 chars
    // long, then just assume its the channel id
    if identifier.len() == 24 {
        Ok(identifier.to_string())
    } else {
        // the channel id comes after `/channel/` in an url
        let index = if let Some(index) = identifier.find("/channel/") {
            // if there isnt 24 characters after `/channel/`, that url must not have
            // contained a channel id
            if identifier.len() < index + 33 {
                return Err(format!(
                    "Cannot find channel id from string `{}`",
                    identifier
                ));
            }
            index + 9
        } else {
            return Err(format!(
                "Cannot find channel id from string `{}`",
                identifier
            ));
        };

        Ok(identifier[index..index + 24].to_string())
    }
}

pub fn from_video_url(identifier: &str) -> Result<String, String> {
    // a youtube video id is exactly 11 characters long, so if the identifier to 11 chars
    // long, then just assume its the video id
    if identifier.len() == 11 {
        Ok(identifier.to_string())
    } else {
        // the id can come after "?v=" in an url
        let index = if let Some(index) = identifier.find("?v=") {
            if identifier.len() < index + 15 {
                return Err(format!("Cannot find video id from string `{}`", identifier));
            }
            index + 3
        // and also "youtu.be/"
        } else if let Some(index) = identifier.find("youtu.be/") {
            if identifier.len() < index + 21 {
                return Err(format!("Cannot find video id from string `{}`", identifier));
            }
            index + 9
        } else {
            return Err(format!("Cannot find video id from string `{}`", identifier));
        };

        Ok(identifier[index..index + 11].to_string())
    }
}

pub fn from_playlist_url(identifier: &str) -> Result<String, String> {
    // a youtube playlist id is exactly 34 characters long, so if the identifier to 34 chars
    // long, then just assume its the playlist id
    if identifier.len() == 34 {
        Ok(identifier.to_string())
    } else {
        // the id can come after "list=" in urls
        let index = if let Some(index) = identifier.find("list=") {
            if identifier.len() < index + 39 {
                return Err(format!(
                    "Cannot find plalyist id from string `{}`",
                    identifier
                ));
            }
            index + 5
        } else {
            return Err(format!(
                "Cannot find playlist id from string `{}`",
                identifier
            ));
        };

        Ok(identifier[index..index + 34].to_string())
    }
}
