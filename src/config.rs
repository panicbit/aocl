use preferences::{Preferences, PreferencesError};
use failure::ResultExt;
use chrono::prelude::*;
use std::io;
use {Result, APP_INFO, Leaderboard};

const CONF_LEADERBOARD_URL: &str = "leaderboard_url";
const CONF_SESSION_TOKEN: &str = "session_token";
const CONF_LAST_API_ACCESS: &str = "last_api_access";
const CONF_LAST_API_RESPONSE: &str = "last_api_response";

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

pub fn last_api_access() -> Result<Option<DateTime<Local>>> {
    let time = match <Option<DateTime<Local>>>::load(APP_INFO, CONF_LAST_API_ACCESS) {
        Err(PreferencesError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => None,
        res => res.context("Failed to load last API access timestamp")?,
    };
    Ok(time)
}

pub fn set_last_api_access(last_access: Option<DateTime<Local>>) -> Result<()> {
    last_access.save(APP_INFO, CONF_LAST_API_ACCESS)
        .context("Failed to save last API access timestamp")?;
    Ok(())
}

pub fn last_leaderboard() -> Result<Option<Leaderboard>> {
    let response = match <Option<Leaderboard>>::load(APP_INFO, CONF_LAST_API_RESPONSE) {
        Err(PreferencesError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => None,
        res => res.context("Failed to load last leaderboard")?,
    };
    Ok(response)
}

pub fn set_last_leaderboard(leaderboard: Leaderboard) -> Result<()> {
    leaderboard.save(APP_INFO, CONF_LAST_API_RESPONSE)
        .context("Failed to save last leaderboard")?;
    Ok(())
}
