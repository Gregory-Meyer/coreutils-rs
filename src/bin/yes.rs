extern crate clap;

use std::io::Write;

fn get_string() -> String {
    "y\n".to_string()
}

fn main() {
    let to_repeat = get_string();

    let repeated: Vec<u8> = to_repeat.as_bytes()
                                     .iter()
                                     .cloned()
                                     .cycle()
                                     .take(4096)
                                     .collect();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let ref slice = &repeated.as_slice();

    loop {
        match lock.write_all(slice) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("error while writing: {}", e);
                std::process::exit(1);
            }
        }
    }
}
