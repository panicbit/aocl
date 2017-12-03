use preferences::Preferences;
use failure::ResultExt;
use {Result, APP_INFO};

const CONF_LEADERBOARD_URL: &str = "leaderboard_url";
const CONF_SESSION_TOKEN: &str = "session_token";

pub fn leaderboard_url() -> Result<String> {
    let url = String::load(APP_INFO, CONF_LEADERBOARD_URL)
        .context("Leaderboard url not set.\n\
                  Set one using `--url https://adventofcode.com/YEAR/leaderboard/private/view/ID`.\n\
                  You can get this URL by viewing your private leaderboard\n\
                  and copying it from your browser's address bar.")?;
    Ok(url)
}

pub fn set_leaderboard_url<U: Into<String>>(url: U) -> Result<()> {
    let mut url = url.into();

    if !url.ends_with(".json") {
        url += ".json";
    }

    url.save(APP_INFO, CONF_LEADERBOARD_URL)
        .context("Failed to save leaderboard URL")?;

    Ok(())
}

pub fn session_token() -> Result<String> {
    let url = String::load(APP_INFO, CONF_SESSION_TOKEN)
        .context("Session token not set.\n\
                  Set one using `--session SESSION_TOKEN`.\n\
                  Get this one from the AoC cookies.\n\
                  It's the value of the key called `session`.\n\
                  How to do this depends on your browser.\n\
                  Use google or ask around if needed.")?;
    Ok(url)
}

pub fn set_session_token<T: Into<String>>(token: T) -> Result<()> {
    token.into().save(APP_INFO, "session_token")
        .context("Failed to save session token")?;
    Ok(())
}
