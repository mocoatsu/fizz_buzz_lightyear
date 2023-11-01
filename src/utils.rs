use std::io::{self, Write};

pub fn write_and_flush(stdout: &mut io::Stdout, message: String) -> io::Result<()> {
    stdout.write(message.as_bytes())?;
    stdout.flush()?;

    Ok(())
}
