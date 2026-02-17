
fn slugify(name: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;
    
    for c in name.to_lowercase().chars() {
        if c.is_alphanumeric() {
            result.push(c);
            last_was_dash = false;
        } else if !last_was_dash {
            result.push('-');
            last_was_dash = true;
        }
    }
    
    result.trim_matches('-').to_string()
}

#[test]
fn test_slugify_basic() {
    assert_eq!(slugify("My Organization"), "my-organization");
}

#[test]
fn test_slugify_with_special_chars() {
    assert_eq!(slugify("Test & Co!"), "test-co");
}

#[test]
fn test_slugify_lowercase() {
    assert_eq!(slugify("UPPERCASE"), "uppercase");
}

#[test]
fn test_slugify_removes_leading_trailing_dashes() {
    assert_eq!(slugify("!!!test!!!"), "test");
}

#[test]
fn test_slugify_preserves_existing_dashes() {
    assert_eq!(slugify("my-org-name"), "my-org-name");
}

#[test]
fn test_slugify_empty_string() {
    assert_eq!(slugify(""), "");
}

#[test]
fn test_slugify_only_special_chars() {
    assert_eq!(slugify("!!!@@@###"), "");
}
