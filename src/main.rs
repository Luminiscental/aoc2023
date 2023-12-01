mod day;

use day::Day;

#[macro_export]
macro_rules! bench_day {
    ($day:literal) => {
        paste::paste! {
            #[cfg(not(debug_assertions))]
            #[cfg(test)]
            mod [<bench_day $day>] {
                use $crate::day::Day;
                use super::*;
                use test::Bencher;

                #[bench]
                fn [<bench_day $day _overall>] (b: &mut Bencher) {
                    let input = [<Day $day>]::get_input().unwrap();
                    b.iter(|| {
                        let input = [<Day $day>]::parse(&input);
                        let (input, part1) = [<Day $day>]::solve_part1(input);
                        let part2 = [<Day $day>]::solve_part2(input);
                        (part1, part2)
                    })
                }
            }
        }
    };
}

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
    01
);
