use std::io::{Read, Write};

fn split_buffer<'a>(buffer: &'a str, split: &str) -> Vec<&'a str> {
    buffer.split_terminator(split).collect()
}

fn reverse_split_buffer<'a>(buffer: &'a str, split: &str) -> Vec<&'a str> {
    buffer.rsplit_terminator(split).collect()
}

fn print_buffers(buffers: &[&str], terminator: &str)
-> std::io::Result<usize> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let mut num_wrote = 0;

    for buffer in buffers.iter() {
        num_wrote += match lock.write(buffer.as_bytes()) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        num_wrote += match lock.write(terminator.as_bytes()) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
    }

    Ok(num_wrote)
}

fn print_buffers_reversed(buffers: &[&str], terminator: &str)
-> std::io::Result<usize> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let mut num_wrote = 0;

    for buffer in buffers.iter().rev() {
        num_wrote += match lock.write(buffer.as_bytes()) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        num_wrote += match lock.write(terminator.as_bytes()) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
    }

    Ok(num_wrote)
}

fn print_buffer(buffer: &str) -> std::io::Result<usize> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    lock.write(buffer.as_bytes())
}

fn read_to_buffer<R: std::io::Read>(readable: R) -> std::io::Result<String> {
    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(readable);

    match reader.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn print_readable<R: std::io::Read>(readable: R)
-> std::io::Result<usize> {
    let buffer = match read_to_buffer(readable) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };

    print_buffer(buffer.as_str())
}

pub fn print_readable_reversed<R: std::io::Read>(readable: R)
-> std::io::Result<usize> {
    let buffer = match read_to_buffer(readable) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };

    let lines = split_buffer(buffer.as_str(), "\n");

    print_buffers_reversed(&lines.as_slice(), "\n")
}

pub fn print_stdin() -> std::io::Result<usize> {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    print_readable(lock)
}

pub fn print_stdin_reversed() -> std::io::Result<usize> {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    print_readable_reversed(lock)
}

pub fn print_sorted<R: std::io::Read>(readable: R) -> std::io::Result<usize> {
    let buffer = match read_to_buffer(readable) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };

    let mut lines = split_buffer(buffer.as_str(), "\n");
    lines.sort_unstable();

    print_buffers(&lines.as_slice(), "\n")
}

pub fn print_sorted_stdin() -> std::io::Result<usize> {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    print_sorted(lock)
}
