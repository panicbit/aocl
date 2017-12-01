extern crate serde;
extern crate reqwest;
extern crate failure;
extern crate chrono;
extern crate itertools;
extern crate term_painter;
extern crate structopt;
extern crate preferences;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt_derive;

use term_painter::ToStyle;
use term_painter::Color::{
    BrightYellow as Gold,
    BrightBlack as Gray,
    BrightBlue as Silver,
};
use structopt::StructOpt;
use preferences::{AppInfo, Preferences};
use failure::ResultExt;
use itertools::Itertools;

mod cli;
mod leaderboard;

use self::cli::Cli;
use leaderboard::Leaderboard;

const APP_INFO: &AppInfo = &AppInfo {
    name: "aocl",
    author: "panicbit",
};

type Result<T> = ::std::result::Result<T, failure::Error>;

fn main() {
    if let Err(e) = result_main() {
        println!("{}", e);
    }
}

fn result_main() -> Result<()> {
    let cli = Cli::from_args();
    cli.update_preferences()?;

    let leaderboard_url = load_leaderboard_url()?;
    let session_token = load_session_token()?;
    let leaderboard = Leaderboard::fetch(&leaderboard_url, &session_token)?;

    print_leaderboard(&leaderboard);

    Ok(())
}

fn load_leaderboard_url() -> Result<String> {
    let url = String::load(APP_INFO, "leaderboard_url")
        .context("Leaderboard url not set.\n\
                  Set one using `--url https://adventofcode.com/YEAR/leaderboard/private/view/ID`.\n\
                  You can get this URL by viewing your private leaderboard\n\
                  and copying it from your browser's address bar.")?;
    Ok(url)
}

fn load_session_token() -> Result<String> {
    let url = String::load(APP_INFO, "session_token")
        .context("Session token not set.\n\
                  Set one using `--session SESSION_TOKEN`.\n\
                  Get this one from the AoC cookies.\n\
                  It's the value of the key called `session`.\n\
                  How to do this depends on your browser.\n\
                  Use google or ask around if needed.")?;
    Ok(url)
}

fn print_leaderboard(leaderboard: &Leaderboard) {
    println!("Advent of Code {} | Leaderboard #{}\n", leaderboard.event(), leaderboard.owner_id());

    // Day label
    {
        print!("                            ");
        for day in 10..26 {
            print!("{} ", Gray.paint(day / 10));
        }
        print!("\n          ");
        for day in 1..26 {
            print!("{} ", Gray.paint(day % 10));
        }
        println!();
    }

    let ranked_members = leaderboard.members()
        .sorted_by(|a, b| b.local_score().cmp(&a.local_score()))
        .into_iter()
        .enumerate()
        .map(|(rank, member)| (rank + 1, member));

    for (rank, member) in ranked_members {
        print!("{: >3})  ", rank);
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
                    .unwrap_or(Gray.paint('*'));
                print!("{} ", star);
            }
        }

        println!("{}", member.name());
    }
}

