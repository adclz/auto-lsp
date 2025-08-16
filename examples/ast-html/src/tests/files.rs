#[cfg(test)]
mod tests {
    use auto_lsp::{default::db::file::File, lsp_types::Url, salsa};

    fn make_html_file(db: &impl salsa::Database, content: &str) -> File {
        // Minimal fake setup: real code may require a salsa DB & parser
        let url = Url::parse("file:///test.txt").unwrap();
        File::from_string()
            .db(db)
            .maybe_encoding(None)
            .source(content.to_string())
            .url(&url)
            .parsers(
                crate::db::HTML_PARSERS
                    .get("html")
                    .expect("Html parser not found"),
            )
            .call()
            .expect("Failed to create file")
    }

    #[test]
    fn test_same_content() {
        let db = salsa::DatabaseImpl::default();
        let file = make_html_file(&db, "hello world");
        assert!(file.fail_fast_check(&db, "hello world"));
    }

    #[test]
    fn test_different_length() {
        let db = salsa::DatabaseImpl::default();
        let file = make_html_file(&db, "hello");
        assert!(!file.fail_fast_check(&db, "hello world"));
    }

    #[test]
    fn test_same_length_different_content() {
        let db = salsa::DatabaseImpl::default();
        let file = make_html_file(&db, "hello");
        assert!(!file.fail_fast_check(&db, "hELlo"));
    }

    #[test]
    fn test_large_equal_content() {
        let db = salsa::DatabaseImpl::default();
        let s1 = "a".repeat(100_000);
        let file = make_html_file(&db, &s1);
        let s2 = "a".repeat(100_000);
        assert!(file.fail_fast_check(&db, &s2));
    }

    #[test]
    fn test_large_different_content() {
        let db = salsa::DatabaseImpl::default();
        let s1 = "a".repeat(100_000);
        let file = make_html_file(&db, &s1);
        let mut s2 = "a".repeat(100_000);
        s2.replace_range(50_000..50_001, "b"); // one char different
        assert!(!file.fail_fast_check(&db, &s2));
    }
}
