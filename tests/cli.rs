#[cfg(test)]
mod tests {
    use crust::cli::parse_arguments;

    #[test]
    fn test_cli_arguments() {
        let args = vec![
            "program".to_string(),
            "--url".to_string(),
            "http://example.com".to_string(),
            "-l".to_string(),
            "5".to_string(),
            "-o".to_string(),
            "output.txt".to_string()
        ];
        let config = parse_arguments(Some(args));
        assert_eq!(config.url, "http://example.com");
        assert_eq!(config.min_length, 5);
        assert_eq!(config.output_file, Some("output.txt".to_string()));
    }
}
