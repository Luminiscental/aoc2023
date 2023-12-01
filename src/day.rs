use reqwest::{self, header::COOKIE};
use std::{
    error,
    fmt::{self, Display, Formatter},
    fs, io,
    time::Instant,
};

#[derive(Debug)]
pub enum Error {
    MissingCookie(io::Error),
    DownloadingInput(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingCookie(err) => write!(
                f,
                "Missing session cookie to access input; please \
                create a text file at cache/.session_cookie containing \
                your session cookie for adventofcode.com [{}]",
                err
            ),
            Self::DownloadingInput(err) => write!(f, "Couldn't download input... [{}]", err),
        }
    }
}

impl error::Error for Error {}

pub trait Day<'a> {
    const DAY: usize;
    type Input;
    type ProcessedInput;

    fn parse(input: &'a str) -> Self::Input;
    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String);
    fn solve_part2(input: Self::ProcessedInput) -> String;

    fn get_input() -> Result<String, Error> {
        let input_path = format!("cache/day{}.in", Self::DAY);
        fs::read_to_string(&input_path).or_else(|_| {
            let cookie = format!(
                "session={}",
                fs::read_to_string("cache/.session_cookie")
                    .map_err(Error::MissingCookie)?
                    .trim()
            );
            let input = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/2023/day/{}/input",
                    Self::DAY
                ))
                .header(COOKIE, cookie)
                .send()
                .map_err(Error::DownloadingInput)?
                .text()
                .map_err(Error::DownloadingInput)?;
            drop(fs::write(input_path, &input));
            Ok(input)
        })
    }

    fn solve_and_print(input: &'a str) {
        println!();
        println!("day{:02}:", Self::DAY);

        let start_time = Instant::now();
        let input = Self::parse(input);
        let parsed_time = Instant::now();
        println!(
            "  parsing: ... (elapsed {}ms)",
            1000.0 * (parsed_time - start_time).as_secs_f32()
        );

        let (processed_input, part1_answer) = Self::solve_part1(input);
        let part1_time = Instant::now();
        println!(
            "  part1: {} (elapsed {}ms)",
            part1_answer,
            1000.0 * (part1_time - parsed_time).as_secs_f32()
        );

        let part2_answer = Self::solve_part2(processed_input);
        let part2_time = Instant::now();
        println!(
            "  part2: {} (elapsed {}ms)",
            part2_answer,
            1000.0 * (part2_time - part1_time).as_secs_f32()
        )
    }
}
