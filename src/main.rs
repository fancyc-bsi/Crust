use std::env;
use log::{info, error};
use crust::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");

    env_logger::init();

    let config = cli::parse_arguments(None);

    info!("Starting fetch for URL: {}", config.url);
    match cewl::fetcher::fetch_url(&config.url).await {
        Ok(html) => {
            info!("Successfully fetched content from URL");
            let words = cewl::parser::extract_words(&html);
            info!("Words extracted and processed");

            let wordlist = cewl::generator::generate_wordlist(words, config.min_length);
            info!("Wordlist generated");

            save::save_or_print_wordlist(wordlist.into_iter(), config.output_file)?;
            info!("Output handling completed");
        },
        Err(e) => {
            error!("Failed to fetch URL: {}", e);
            return Err(e.into());
        },
    }
    Ok(())
}
