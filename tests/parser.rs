#[cfg(test)]
mod tests {
    use crust::cewl::parser::extract_words;

    #[test]
    fn test_extract_simple_words() {
        let html = "<html><body>Hello world, this is a test. <script>ignore this</script><style>and this</style></body></html>";
        let words = extract_words(html);
        assert_eq!(words, vec!["hello", "world", "this", "is", "a", "test"]);
    }
}
