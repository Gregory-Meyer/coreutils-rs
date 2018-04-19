extern crate clap;
extern crate coreutils;

use std::error::Error;

enum File {
    Named(String),
    Stdin,
}

impl File {
    fn from_str(name: &str) -> File {
        if name == "-" {
            File::Stdin
        } else {
            File::Named(String::from(name))
        }
    }
}

fn get_files() -> Vec<File> {
    let args = clap::App::new("sort")
        .version("0.1.0")
        .author("Gregory Meyer <gregjm@umich.edu>")
        .about("sort text files")
        .arg(clap::Arg::with_name("file")
             .multiple(true))
        .get_matches();

    match args.values_of("file") {
        Some(files) => {
            files.map(File::from_str).collect()
        }
        None => {
            vec![File::Stdin]
        }
    }
}

fn main() {
    let files = get_files();

    for file in files {
        match file {
            File::Named(filename) => {
                let file = match std::fs::File::open(&filename) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("unable to open '{}': {}", filename,
                                  e.description());
                        std::process::exit(1);
                    }
                };

                match coreutils::print_sorted(file) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("error: {}", e.description());
                        std::process::exit(1);
                    }
                }
            }
            File::Stdin => match coreutils::print_sorted_stdin() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("error: {}", e.description());
                    std::process::exit(1);
                }
            }
        }
    }
}
