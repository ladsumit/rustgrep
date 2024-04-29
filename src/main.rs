use clap::{App, Arg};
mod search;

fn main() {
    let matches = App::new("RustGrep")
        .version("1.0")
        .author("Sumit Lad")
        .about("Searches for patterns in files")
        .arg(Arg::with_name("pattern")
            .short('p')
            .long("pattern")
            .takes_value(true)
            .required(true)
            .help("Pattern to search for"))
        .arg(Arg::with_name("path")
            .short('d')
            .long("path")
            .takes_value(true)
            .required(true)
            .help("Directory to search in"))
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let path = matches.value_of("path").unwrap();

    println!("Searching for '{}' in '{}'", pattern, path);
    if let Err(e) = search::search_directory(pattern, path) {
        eprintln!("Error occurred: {}", e);
    }
}
