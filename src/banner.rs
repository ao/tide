pub fn banner() -> &'static str {
    r#"
████████╗██╗██████╗ ███████╗
╚══██╔══╝██║██╔══██╗██╔════╝
   ██║   ██║██║  ██║█████╗  
   ██║   ██║██║  ██║██╔══╝  
   ██║   ██║██████╔╝███████╗
   ╚═╝   ╚═╝╚═════╝ ╚══════╝                                 
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_not_empty() {
        let banner_text = banner();
        assert!(!banner_text.is_empty());
    }

    #[test]
    fn test_banner_contains_expected_content() {
        let banner_text = banner();
        // Check for some expected content in the ASCII art
        assert!(banner_text.contains("██████╗██╗███"));
        assert!(banner_text.contains("██║█████"));
    }

    #[test]
    fn test_banner_is_multiline() {
        let banner_text = banner();
        let line_count = banner_text.lines().count();
        // The banner should have multiple lines (at least 5)
        assert!(line_count >= 5);
    }
}
