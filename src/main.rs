mod fizz_buzz;
mod utils;

use std::io::{self};
use std::{thread, time};

use fizz_buzz::run_fizz_buzz;
pub use utils::write_and_flush;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let _ = run_fizz_buzz();

    for i in 100..=10000 {
        write_and_flush(&mut stdout, format!("\r{}", i))?;

        thread::sleep(time::Duration::from_nanos(10));
    }

    write_and_flush(&mut stdout, "\r                       ".to_string())?;
    write_and_flush(&mut stdout, "\rTo infinity and beyond!".to_string())?;

    Ok(())
}
