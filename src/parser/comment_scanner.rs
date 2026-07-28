use regex::Regex;
use std::sync::OnceLock;

/// `CompiledRegexes` is a collection of regex patterns, to be lazy-initialized once.
struct CompiledRegexes {
    name: Regex,
    author: Regex,
    strategy: Regex,
}

/// `CommentKeyPattern` provides an interface to indicate the specific comment metadata pattern to look for.
#[derive(Debug, Clone, Copy)]
pub enum CommentKeyPattern {
    Name,
    Author,
    Strategy,
}

/// Scan `comment` to look for a `key`, and return the value if it is found.
pub fn scan_comment(key: CommentKeyPattern, comment: &str) -> Option<&str> {
    static REGEXES: OnceLock<CompiledRegexes> = OnceLock::new();

    #[allow(
        clippy::missing_panics_doc,
        reason = "These regex definitions are valid."
    )]
    let re = REGEXES.get_or_init(|| CompiledRegexes {
        // Regex: The pattern must start with ";name", followed by one space or tab.
        name: Regex::new("^;name[ \t]").unwrap(),

        // Regex: The pattern must start with ";author", followed by one space or tab.
        author: Regex::new("^;author[ \t]").unwrap(),

        // Regex: The pattern must start with ";strategy", followed by one space or tab.
        strategy: Regex::new("^;strategy[ \t]").unwrap(),
    });

    match key {
        CommentKeyPattern::Name => find_value(&re.name, comment),
        CommentKeyPattern::Author => find_value(&re.author, comment),
        CommentKeyPattern::Strategy => find_value(&re.strategy, comment),
    }
}

/// If `text` contains a match for `key_regex`, return the remaining non-whitespace string if possible.
fn find_value<'a>(key_regex: &Regex, text: &'a str) -> Option<&'a str> {
    key_regex
        .find(text)
        .map(|mat| text[mat.end()..].trim())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use CommentKeyPattern::*;

    #[test]
    fn test_scan_comment_should_return_some() {
        assert_eq!(scan_comment(Name, ";name Big Doge"), Some("Big Doge"));

        assert_eq!(
            scan_comment(Author, ";author  John Smith  "),
            Some("John Smith")
        );

        assert_eq!(
            scan_comment(Strategy, ";strategy I don't know."),
            Some("I don't know.")
        );
    }

    #[test]
    fn test_scan_comment_should_return_none() {
        assert_eq!(scan_comment(Name, "My ;name is Big Doge"), None);

        assert_eq!(scan_comment(Author, ";;author John"), None);
        assert_eq!(scan_comment(Author, ";authorJohn"), None);

        assert_eq!(scan_comment(Strategy, ";strategy"), None);
        assert_eq!(scan_comment(Strategy, ";strategy "), None);
        assert_eq!(scan_comment(Strategy, ";strategy  "), None);
        assert_eq!(scan_comment(Strategy, ";strategy   "), None);
    }
}
