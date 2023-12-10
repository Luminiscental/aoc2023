mod day;
mod util;

use day::Day;

macro_rules! import_days {
    ($day:literal) => {
        paste::paste! {
            mod [<day $day>];
            use [<day $day>]::[<Day $day>];
        }
    };
    ($day:literal, $($days:literal),+) => {
        import_days!($day);
        import_days!($($days),+);
    }
}

macro_rules! solve {
    ($day:literal) => {{
        paste::paste! {
            match [<Day $day>]::get_input() {
                Ok(input) => [<Day $day>]::solve_and_print(&input),
                Err(err) => eprintln!("{}", err),
            }
        }
    }};
    ($day:literal, $($days:literal),+) => {{
        solve!($day);
        solve!($($days),+)
    }}
}

macro_rules! match_days {
    ($day_string:ident, $($days:literal),+) => {{
        match $day_string {
            "all" => solve!($($days),+),
            day => match day.parse::<usize>() {
                Err(err) => eprintln!("Expected day number (or \"all\") as argument ({})", err),
                $(Ok($days) => solve!($days)),+,
                Ok(n) if (1..=25).contains(&n) => todo!(),
                Ok(_) => eprintln!("That's not a day of advent!"),
            }
        }
    }}
}

macro_rules! declare_main {
    ($last_day:literal => $($days:literal),+) => {
        import_days!($($days),+);
        fn main() {
            #[allow(clippy::zero_prefixed_literal)]
            match std::env::args().nth(1).as_deref() {
                None => solve!($last_day),
                Some(day) => match_days!(day, $($days),+),
            }
        }
    };
    ($head_day:literal, $($last_days:literal),+ => $($days:literal),+) => {
        declare_main!($($last_days),+ => $($days),+);
    };
    ($($days:literal),+) => {
        declare_main!($($days),+ => $($days),+);
    }
}

declare_main!(
    01, 02, 03, 04, 05, 06, 07, 08, 09, 10
);
