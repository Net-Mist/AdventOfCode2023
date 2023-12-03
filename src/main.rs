pub mod helper;

use std::{
    cmp::min,
    time::{Duration, Instant},
};

use aoc_macro::main;
use argh::FromArgs;
use colored::*;

use tikv_jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(FromArgs)]
/** Advent of Code (https://adventofcode.com/)
*/
struct Args {
    /// A single day to execute (all days by default)
    #[argh(option, short = 'd')]
    day: Option<usize>,
}

fn pretty_print(line: &str, output: Option<&str>, duration: Duration) {
    const DISPLAY_WIDTH: usize = 40;

    let duration = format!("({:.2?})", duration);
    print!("{} {}", line, duration.dimmed());

    match output {
        Some(output) => {
            let width = "  - ".len() + line.chars().count() + 1 + duration.chars().count();
            let dots = DISPLAY_WIDTH - min(DISPLAY_WIDTH - 5, width) - 2;
            print!(" {}", ".".repeat(dots).dimmed());

            if output.contains('\n') {
                println!();

                for line in output.trim_matches('\n').lines() {
                    println!("    {}", line.bold());
                }
            } else {
                println!(" {}", output.bold());
            }
        }
        None => println!(),
    }
}

// Time the given function, returning its result and the elapsed time
fn time<T>(func: &dyn Fn() -> T) -> (Duration, T) {
    let start = Instant::now();
    let result = func();
    (start.elapsed(), result)
}

main!(25);
