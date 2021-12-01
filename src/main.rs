use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use dotenv::dotenv;

mod days;
mod utils;

#[derive(Parser)]
struct Opts {
    /// The day to run
    day: u8,

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

    let data = read_data(args.year, args.day, args.data_path, args.aoc_session)
        .expect("Could not read the data file");
    let day = days::DAYS[args.day as usize - 1];

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("~~~ Advent of Code {} ~~~", args.year);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("# Day {}", args.day);
    println!("q1 = {}", day.q1(&data));
    println!("q2 = {}", day.q2(&data));

    Ok(())
}

struct Args {
    year: u16,
    day: u8,
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
    let aoc_session = opts.session.or(env::var("AOC_SESSION").ok());

    Ok(Args {
        year: 2021,
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
    aoc_session: Option<String>,
) -> anyhow::Result<String> {
    const COMPRESSION: i32 = 21;
    let path = data_path.as_ref().join(format!("day{}.zst", day));
    let parent = path.parent().unwrap();

    std::fs::create_dir_all(&parent)
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
fn download_input(year: u16, day: u8, aoc_session: String) -> anyhow::Result<String> {
    let url = utils::get_input_url(year, day);
    let body = ureq::get(&url)
        .set("Cookie", &format!("session={}", aoc_session))
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
