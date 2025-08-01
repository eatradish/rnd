use std::{
    ops::{Range, RangeInclusive},
    str::FromStr,
};

use clap::Parser;
use num::{Bounded, Num};
use rand::{Rng, distr::uniform::SampleUniform, rng, rngs::ThreadRng};

#[derive(Debug, Parser)]
struct App {
    /// Random result contains float number
    #[arg(short, long)]
    float: bool,
    /// Get result count
    #[arg(short = 'n', long, default_value_t = 1)]
    count: usize,
    /// Input range
    #[arg(allow_hyphen_values = true)]
    range: Option<String>,
}

macro_rules! print_result {
    ($t:ident, $range:ident, $n:ident) => {
        let range = parse_range::<$t>(&$range).expect("Should parse success");
        let mut rng = rng();
        for _ in 0..$n {
            let res: $t = range.clone().random_range(&mut rng);
            println!("{res}");
        }
    };
    ($t:ident, $n:ident) => {
        let mut rng = rng();
        for _ in 0..$n {
            let res: $t = rng.random();
            println!("{res}");
        }
    };
}

fn main() {
    let App {
        float,
        range,
        count,
    } = App::parse();

    if let Some(range) = range {
        if float {
            print_result!(f64, range, count);
        } else {
            print_result!(i64, range, count);
        }
    } else if float {
        print_result!(f64, count);
    } else {
        print_result!(i64, count);
    }
}

#[derive(Debug, Clone)]
enum RangeType<T> {
    Range(Range<T>),
    RangeInclusive(RangeInclusive<T>),
}

impl<T: SampleUniform + PartialOrd> RangeType<T> {
    fn random_range(self, rng: &mut ThreadRng) -> T {
        match self {
            RangeType::Range(range) => rng.random_range(range),
            RangeType::RangeInclusive(range) => rng.random_range(range),
        }
    }
}

fn parse_range<T>(range: &str) -> Option<RangeType<T>>
where
    T: FromStr + Num + Bounded,
{
    let (start, end) = range.split_once("..")?;

    let start = if start.is_empty() {
        T::min_value()
    } else {
        start.parse().ok()?
    };

    let mut is_eq = false;
    let end = if end.is_empty() {
        T::max_value()
    } else if let Some(end) = end.strip_prefix('=') {
        is_eq = true;
        end.parse().ok()?
    } else {
        end.parse().ok()?
    };

    Some(if is_eq {
        RangeType::RangeInclusive(RangeInclusive::new(start, end))
    } else {
        RangeType::Range(Range { start, end })
    })
}
