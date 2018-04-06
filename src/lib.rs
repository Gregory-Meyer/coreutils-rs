use std::io::{Read, Write};

pub fn print_file<R: std::io::Read>(file: R) -> std::io::Result<usize> {
    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(file);

    match reader.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    
    lock.write(buffer.as_bytes())
}

pub fn print_stdin() -> std::io::Result<usize> {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    print_file(lock)
}
