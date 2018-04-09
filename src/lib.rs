use std::io::{Read, Write};

fn print_buffer(buffer: &str) -> std::io::Result<usize> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    lock.write(buffer.as_bytes())
}

fn reverse_buffer_lines(buffer: &str) -> String {
    let reversed: Vec<_> = buffer.rsplit_terminator('\n')
        .chain([""].into_iter().cloned())
        .collect();

    reversed.join("\n")
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

    let reversed = reverse_buffer_lines(buffer.as_str());

    print_buffer(reversed.as_str())
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
