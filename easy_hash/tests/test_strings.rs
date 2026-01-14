use easy_hash::EasyHash;
use test_case::test_case;

#[test_case("hello" ; "for hello")]
#[test_case("" ; "for empty string")]
#[test_case("a" ; "for single char")]
#[test_case("hello world" ; "for string with space")]
#[test_case("!@#$%^&*()" ; "for special chars")]
#[test_case("你好" ; "for unicode")]
#[test_case("a\nb\tc" ; "for escape sequences")]
#[test_case("\0" ; "for null byte")]
#[test_case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" ; "for long string")]
fn test_str_special_cases(s: &str) {
    // Test that the hash is consistent for the same value
    assert_eq!(s.ehash(), s.ehash());

    // Test that different strings have different hashes
    for other in ["", "a", "hello", "world", "你好", "\0"] {
        if s != other {
            assert_ne!(s.ehash(), other.ehash());
        }
    }

    // Test that the hash is different from a string with the same first char
    if !s.is_empty() {
        let first_char = s.chars().next().unwrap().to_string();
        if s != first_char.as_str() {
            assert_ne!(s.ehash(), first_char.ehash());
        }
    }

    // Test that the hash is different from a prefix of the string
    if s.len() > 1 {
        let prefix = &s[0..s.len() / 2];
        assert_ne!(s.ehash(), prefix.ehash());
    }
}

#[test]
fn test_str_case_sensitivity() {
    let lowercase = "hello";
    let uppercase = "HELLO";
    assert_ne!(lowercase.ehash(), uppercase.ehash());
}

#[test]
fn test_str_vs_string() {
    let s_str = "test string";
    let s_string = String::from("test string");
    assert_ne!(s_str.ehash(), s_string.ehash());
}

#[test]
fn test_str_slices() {
    let s = "hello world";
    let slice1 = &s[0..5];
    let slice2 = &s[6..];

    assert_ne!(s.ehash(), slice1.ehash());
    assert_ne!(s.ehash(), slice2.ehash());
    assert_ne!(slice1.ehash(), slice2.ehash());
}

#[test]
fn test_identical_looking_but_different_unicode() {
    // Cyrillic 'а' (U+0430) vs Latin 'a' (U+0061)
    let s1 = "а";
    let s2 = "a";
    assert_ne!(s1.ehash(), s2.ehash());

    // Regular space vs non-breaking space
    let s3 = " ";
    let s4 = "\u{00A0}";
    assert_ne!(s3.ehash(), s4.ehash());
}
