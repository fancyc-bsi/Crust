use clap::{App, Arg};

pub struct Config {
    pub url: String,
    pub min_length: usize,
    pub output_file: Option<String>,
}

pub fn parse_arguments(args: Option<Vec<String>>) -> Config {
    let app = App::new("Rust CEWL")
        .version("0.1.0")
        .author("Your Name")
        .about("Generates a custom wordlist from URLs")
        .arg(Arg::with_name("URL")
            .help("The URL to fetch data from")
            .required(true)
            .long("url")  // This specifies the option as --url
            .takes_value(true))
        .arg(Arg::with_name("min_length")
            .short("l")
            .long("min-length")
            .help("Minimum length of words to include")
            .takes_value(true)
            .default_value("3"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("File to write the wordlist to")
            .takes_value(true));

    let matches = if let Some(args) = args {
        app.get_matches_from(args)
    } else {
        app.get_matches()
    };

    Config {
        url: matches.value_of("URL").unwrap().to_string(),
        min_length: matches.value_of("min_length").unwrap_or("3").parse().unwrap(),
        output_file: matches.value_of("output").map(|s| s.to_string()),
    }
}
