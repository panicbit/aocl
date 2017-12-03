extern crate serde;
extern crate reqwest;
extern crate failure;
extern crate chrono;
extern crate chrono_tz;
extern crate chrono_humanize;
extern crate itertools;
extern crate term_painter;
extern crate structopt;
extern crate preferences;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt_derive;

use term_painter::ToStyle;
use term_painter::Color::{
    BrightYellow as Gold,
    BrightBlack as Gray,
    BrightBlue as Silver,
    NotSet as White,
};
use structopt::StructOpt;
use preferences::AppInfo;
use itertools::Itertools;
use chrono::prelude::*;
use chrono::Duration;
use chrono_humanize::Humanize;

mod cli;
mod leaderboard;
mod config;

use self::cli::Cli;
use leaderboard::Leaderboard;

const APP_INFO: &AppInfo = &AppInfo {
    name: "aocl",
    author: "panicbit",
};

lazy_static! {
    static ref DEFAULT_CACHE_TIMEOUT: Duration = Duration::minutes(15);
}

type Result<T> = ::std::result::Result<T, failure::Error>;

fn main() {
    if let Err(e) = result_main() {
        println!("{}", e);
    }
}

fn result_main() -> Result<()> {
    let cli = Cli::from_args();
    cli.update_preferences()?;

    let leaderboard = get_leaderboard()?;
    print_leaderboard(&leaderboard)?;

    Ok(())
}

fn get_leaderboard() -> Result<Leaderboard> {
    let now = Local::now();
    let last_access = config::last_api_access()?;
    let last_leaderboard = config::last_leaderboard()?;

    if let (Some(last_access), Some(leaderboard)) = (last_access, last_leaderboard) {
        let time_passed = now.signed_duration_since(last_access);
        if Duration::zero() <= time_passed && time_passed <= *DEFAULT_CACHE_TIMEOUT {
            return Ok(leaderboard)
        }
    }

    let leaderboard_url = config::leaderboard_url()?;
    let session_token = config::session_token()?;
    let leaderboard = Leaderboard::fetch(&leaderboard_url, &session_token)?;
    config::set_last_api_access(Some(now))?;
    config::set_last_leaderboard(leaderboard.clone())?;

    Ok(leaderboard)
}

fn print_leaderboard(leaderboard: &Leaderboard) -> Result<()> {
    println!("\n Advent of Code {} | Leaderboard #{}\n", leaderboard.event(), leaderboard.owner_id());
    let num_days_unlocked = leaderboard.num_unlocked_days().unwrap_or(25);
    // Day label
    {
        let color = |day| {
            if day <= num_days_unlocked {
                White
            } else {
                Gray
            }
        };
        print!("                             ");
        for day in 10..26 {
            print!("{} ", color(day).paint(day / 10));
        }
        print!("\n           ");
        for day in 1..26 {
            print!("{} ", color(day).paint(day % 10));
        }
        println!();
    }

    let ranked_members = leaderboard.members()
        .sorted_by(|a, b| b.local_score().cmp(&a.local_score()))
        .into_iter()
        .enumerate()
        .map(|(rank, member)| (rank + 1, member));

    for (rank, member) in ranked_members {
        print!(" {: >3})  ", rank);
        print!("{: >3} ", member.local_score());

        {
            let days = member.completed_days();
            for i in 1..26 {
                let star = days.get(&i.to_string())
                    .map(|day|
                        if day.two().is_some() {
                            Gold.paint('*')
                        } else {
                            Silver.paint('*')
                        }
                    )
                    .unwrap_or(
                        if i <= num_days_unlocked {
                            Gray.paint('*')
                        }
                        else {
                            White.paint(' ')
                        }
                    );
                print!("{} ", star);
            }
        }

        println!("{}", member.name());
    }

    // Next unlock
    {
        let duration = leaderboard.duration_until_next_unlock()?;
        if let Some(duration) = duration {
            let mut rest = duration.num_seconds();

            let hours = duration.num_seconds() / 3600;
            rest = rest % 3600;

            let minutes = rest / 60;
            rest = rest % 60;

            let seconds = rest;
            print!("\n Day {day} unlocks in {h:02}:{m:02}:{s:02}",
                day = num_days_unlocked + 1,
                h = hours,
                m = minutes,
                s = seconds
            );
            if let Some(date) = leaderboard.next_unlock_date()? {
                print!(" at {}", date.format("%H:%M"));
            }
            println!();
        }
    }

    // Updated
    if let Ok(Some(last_access)) = config::last_api_access() {
        println!(" Last updated {}.\n", last_access.humanize());
    }

    Ok(())
}

