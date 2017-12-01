use reqwest::Client;
use reqwest::header::Cookie;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use Result;

#[derive(Deserialize,Debug)]
pub struct Leaderboard {
    owner_id: String,
    event: String,
    members: BTreeMap<String, Member>,
}

impl Leaderboard {
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn fetch(leaderboard_url: &str, session_token: &str) -> Result<Leaderboard> {

        let client = Client::new();
        let mut cookie = Cookie::new();
        cookie.append("session", session_token.to_owned());

        let mut resp = client
            .get(leaderboard_url)
            .header(cookie)
            .send()?;

        let leaderboard = resp.json::<Leaderboard>()?;

        Ok(leaderboard)
    }

    pub fn members<'a>(&'a self) -> Box<Iterator<Item=&'a Member> + 'a> {
        Box::new(self.members.values())
    }
}

#[derive(Deserialize,Debug)]
pub struct Member {
    id: String,
    name: String,
    global_score: u32,
    local_score: u32,
    stars: u32,
    completion_day_level: BTreeMap<String, Level>,
    last_star_ts: DateTime<Utc>,
}

impl Member {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn completed_days(&self) -> &BTreeMap<String, Level> {
        &self.completion_day_level
    }

    pub fn local_score(&self) -> u32 {
        self.local_score
    }
}

#[derive(Deserialize,Debug)]
pub struct Level {
    #[serde(rename="1")]
    one: StarInfo,
    #[serde(rename="2")]
    two: Option<StarInfo>,
}

impl Level {
    pub fn one(&self) -> &StarInfo {
        &self.one
    }

    pub fn two(&self) -> Option<&StarInfo> {
        self.two.as_ref()
    }
}

#[derive(Deserialize,Debug)]
pub struct StarInfo {
    get_star_ts: DateTime<Utc>,
}
