use {Result, config};

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
        if let Some(ref url) = self.url {
            config::set_leaderboard_url(url.as_str())?;
        }

        if let Some(ref session) = self.session {
            config::set_session_token(session.as_str())?
        }

        Ok(())
    }
}
