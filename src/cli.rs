use clap::{App,Arg,ArgMatches,SubCommand};
use {aoc, Result, config};

pub fn run() -> Result<()> {
    let cli = App::new("aocl")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .help("Set private leaderboard URL. \
                   You can get this URL by viewing your private leaderboard \
                   and copying it from your browser's address bar.")
        )
        .subcommand(aoc::cli::new_config_subcommand())
        .subcommand(SubCommand::with_name("times")
            .about("Show the amount of time taken to solve a day")
        )
        .get_matches();

    update_preferences(&cli)?;

    match cli.subcommand() {
        (aoc::cli::CONFIG_SUBCOMMAND, Some(args)) => aoc::cli::run_config_subcommand(args),
        ("times", _) => ::cmd_times(),
        _ => ::cmd_default(),
    }
}

fn update_preferences(args: &ArgMatches) -> Result<()> {
    if let Some(url) = args.value_of("url") {
        config::set_leaderboard_url(url)?;
    }

    if let Some(session) = args.value_of("session") {
        config::set_session_token(session)?
    }

    Ok(())
}
