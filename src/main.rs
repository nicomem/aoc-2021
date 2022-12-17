use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::Parser;
use dotenv::dotenv;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

mod utils;
mod y2021;
mod y2022;

#[derive(Parser)]
struct Opts {
    /// The year to use, If not specified, use the current year
    year: Option<u16>,

    /// The day to run. If not specified, run all days
    day: Option<u8>,

    /// Directory containing the data files, or where they will be downloaded to.
    /// Overrides the `DATA_PATH` environment variable.
    #[clap(short, long)]
    data: Option<PathBuf>,

    /// AoC Cookie session identifier, used to download your user input data.
    /// Overrides the `AOC_SESSION` environment variable.
    #[clap(short, long)]
    session: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = config_args()?;

    // Run either one or all days
    let days = if let Some(day) = args.day {
        day..=day
    } else {
        1..=25
    };

    println!("{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~".fg::<Blue>());
    println!(
        "{} {} {} {}",
        "~~~".fg::<Blue>(),
        "Advent of Code".fg::<Cyan>(),
        args.year.fg::<Green>(),
        "~~~".fg::<Blue>()
    );
    println!("{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~".fg::<Blue>());

    let now = OffsetDateTime::now_utc();
    let mut total_duration = Duration::ZERO;
    for day in days {
        println!();

        // Build the release date of the wanted day
        let release = PrimitiveDateTime::new(
            Date::from_calendar_date(args.year as _, time::Month::December, day).unwrap(),
            Time::from_hms(5, 0, 0).unwrap(),
        )
        .assume_utc();

        // If the day challenge has not been released, directly exit without trying to download/run it
        if release > now {
            println!(
                "{} {} {}",
                "Day".fg::<Red>(),
                day.fg::<Yellow>(),
                "challenge has not been released yet!".fg::<Red>()
            );
            break;
        }
        println!("{} {}", "# Day".fg::<Blue>(), day.fg::<Green>());

        let data = read_data(args.year, day, &args.data_path, args.aoc_session.as_deref())
            .expect("Could not read the data file");
        let solution = match args.year {
            2021 => y2021::DAYS,
            2022 => y2022::DAYS,
            _ => {
                println!(
                    "{}",
                    "You cannot go into the future! (or the code has not yet been updated)"
                        .fg::<Red>()
                );
                return Ok(());
            }
        }[day as usize - 1];

        let print_result = |res: &str, dur: Duration| {
            println!(
                "{} {}{} {}{}{}{}",
                "R =".fg::<Cyan>(),
                if res.contains('\n') { "\n" } else { "" }, // For multi-line results, align them all
                res.fg::<Yellow>(),
                "(".fg::<Blue>(),
                dur.as_millis().fg::<Green>(),
                " ms".fg::<Green>(),
                ")".fg::<Blue>(),
            );
        };

        let print_todo = || {
            println!("{} {}", "R =".fg::<Cyan>(), "TODO".fg::<Red>(),);
        };

        let (r, dur) = timer(|| solution.q1(&data));
        if r.is_empty() {
            print_todo();
        } else {
            print_result(&r, dur);
            total_duration += dur;
        }

        let (r, dur) = timer(|| solution.q2(&data));
        if r.is_empty() {
            print_todo();
        } else {
            print_result(&r, dur);
            total_duration += dur;
        }
    }

    println!(
        "\n{} {} {}{}",
        "==>".fg::<Blue>(),
        "Total duration:".fg::<Cyan>(),
        total_duration.as_millis().fg::<Green>(),
        "ms".fg::<Green>()
    );

    Ok(())
}

/// Time a function call
fn timer<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let r = f();
    let end = Instant::now();

    (r, end - start)
}

struct Args {
    year: u16,
    day: Option<u8>,
    data_path: PathBuf,
    aoc_session: Option<String>,
}

fn config_args() -> anyhow::Result<Args> {
    // Load the potential .env file
    if dotenv().is_err() {
        println!("Could not read .env file. Running with only command line arguments.");
    }

    // Read the command line arguments
    let opts: Opts = Opts::parse();

    // Get data path from cmd args or else from env
    let data_path = if let Some(data) = opts.data {
        data
    } else {
        env::var("DATA_PATH")
            .context("Env variable DATA_PATH not found")?
            .into()
    };

    // Get AoC session from cmd args or else from env
    let aoc_session = opts.session.or_else(|| env::var("AOC_SESSION").ok());

    Ok(Args {
        year: opts.year.unwrap_or(2022),
        day: opts.day,
        data_path,
        aoc_session,
    })
}

/// Read the stored data of a day & question.
/// If it does not exists and the AOC session cookie value is given, download it from the AOC servers.
fn read_data(
    year: u16,
    day: u8,
    data_path: impl AsRef<Path>,
    aoc_session: Option<&str>,
) -> anyhow::Result<String> {
    const COMPRESSION: i32 = 21;
    let path = data_path.as_ref().join(format!("day{day}.zst"));
    let parent = path.parent().unwrap();

    std::fs::create_dir_all(parent)
        .with_context(|| format!("Could not create directories '{}'", parent.display()))?;

    let file = if let Ok(file) = File::open(&path) {
        file
    } else {
        // Could not open file, try to download it
        let session = aoc_session.with_context(|| {
            format!(
                "Data file '{}' is not present and no AOC session cookie was given to download it",
                path.display()
            )
        })?;
        let data = download_input(year, day, session).context("Could not download data file")?;

        // Write the data to the file
        {
            let file = File::create(&path)
                .with_context(|| format!("Could not create data file {}", path.display()))?;

            let mut encoder = zstd::Encoder::new(file, COMPRESSION)
                .context("Could not create zstd encoder")?
                .auto_finish();

            encoder
                .write_all(data.as_bytes())
                .context("Could not write data to the file")?;
        }

        File::open(path).context("Could not open data file after writing it")?
    };

    let res = zstd::decode_all(file).context("Could not decode zstd encoded data")?;

    String::from_utf8(res).context("Data is not UTF-8")
}

/// Download an input from the AOC servers for the user with the given session cookie
fn download_input(year: u16, day: u8, aoc_session: &str) -> anyhow::Result<String> {
    let url = utils::get_input_url(year, day);
    let body = ureq::get(&url)
        .set("Cookie", &format!("session={aoc_session}"))
        .call()?
        .into_string()?;

    Ok(body)
}

/// The solution for a day.
trait Solution {
    #[must_use]
    fn q1(&self, data: &str) -> String;

    #[must_use]
    fn q2(&self, data: &str) -> String;
}
