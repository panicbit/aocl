use preferences::Preferences;
use failure::ResultExt;
use {Result, APP_INFO};

#[derive(StructOpt, Debug)]
#[structopt(name = "aocl", about = "Advent of Code private leaderboard viewer")]
pub struct Cli {
    #[structopt(short = "u", long = "url", help = "Set private leaderboard URL. \
                                                   You can get this URL by viewing your private leaderboard \
                                                   and copying it from your browser's address bar.")]
    url: Option<String>,
    #[structopt(short = "s", long = "session", help = "Set the session token. \
                                                       Get this one from the AoC cookies. \
                                                       It's the value of the key called `session`. \
                                                       How to do this depends on your browser. \
                                                       Use google or ask around if needed.")]
    session: Option<String>,
}

impl Cli {
    pub fn update_preferences(&self) -> Result<()> {
        if let Some(mut url) = self.url.as_ref().cloned() {
            if !url.ends_with(".json") {
                url += ".json";
            }

            url.save(APP_INFO, "leaderboard_url")
                .context("Failed to save leaderboard URL")?;
        }

        if let Some(ref session) = self.session {
            session.save(APP_INFO, "session_token")
                .context("Failed to save session token")?;
        }

        Ok(())
    }
}
