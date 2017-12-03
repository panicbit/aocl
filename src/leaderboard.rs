use reqwest::Client;
use reqwest::header::Cookie;
use std::collections::BTreeMap;
use failure::ResultExt;
use chrono::{DateTime, Utc, TimeZone, FixedOffset, Duration};
use Result;

#[derive(Serialize,Deserialize,Debug,Clone)]
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

    fn year(&self) -> Result<u32> {
        let year = self.event.parse::<u32>()
            .context("Event name is not a valid year")?;
        Ok(year)
    }

    pub fn num_unlocked_days(&self) -> Result<u32> {
        let year = self.year()?;
        let december_start = FixedOffset::west(5 * 60 * 60).ymd(year as i32, 12, 1);
        let days = Utc::today().signed_duration_since(december_start).num_days() + 1;

        if days <= 0 {
            Ok(0)
        }
        else if days > 25 {
            Ok(25)
        }
        else {
            Ok(days as u32)
        }
    }

    pub fn duration_until_next_unlock(&self) -> Result<Option<Duration>> {
        let year = self.year()?;
        let num_unlocked_days = self.num_unlocked_days()?;
        let next_locked_day = num_unlocked_days + 1;

        if next_locked_day > 25 {
            return Ok(None);
        }

        let next_locked_day = FixedOffset::west(5 * 60 * 60).ymd(year as i32, 12, next_locked_day).and_hms(0, 0, 0);
        let duration = next_locked_day.signed_duration_since(Utc::now());

        Ok(Some(duration))
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
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

#[derive(Serialize,Deserialize,Debug,Clone)]
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

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct StarInfo {
    get_star_ts: DateTime<Utc>,
}
