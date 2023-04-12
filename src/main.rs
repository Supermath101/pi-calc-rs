use malachite::{
    num::{
        arithmetic::traits::*,
        conversion::{string::options::ToSciOptions, traits::ToSci},
    },
    rounding_modes::RoundingMode,
    Natural, Rational,
};
use rayon::prelude::*;
use std::io::{stdin, BufRead};

const ITERATIONS_PER_CHECK: u64 = 64;

fn main() {
    let mut input_str = String::new();

    let mut sci_options = ToSciOptions::default();
    let mut stdin = stdin().lock();
    println!("How many digits?");
    stdin.read_line(&mut input_str).unwrap();
    sci_options.set_rounding_mode(RoundingMode::Floor);
    sci_options.set_precision(input_str.trim_end().parse().unwrap());
    calc(sci_options);
    println!("Quiting...")
}

pub fn calc(sci_options: ToSciOptions) {
    let mut half_pi = Rational::default();
    let mut prev_pi = String::new();
    let mut n = 0u64;
    loop {
        half_pi += (n..(n + ITERATIONS_PER_CHECK))
            .into_par_iter()
            .map(|i| {
                Rational::from_naturals(
                    Natural::factorial(i),
                    Natural::double_factorial((2 * i) + 1),
                )
            })
            .sum::<Rational>();
        n += ITERATIONS_PER_CHECK;
        let pi = half_pi.clone() * Rational::from(2);
        let pi = pi.to_sci_with_options(sci_options).to_string();
        if pi == prev_pi {
            println!("pi is {}", pi);
            break;
        } else {
            prev_pi = pi;
        }
    }
}
