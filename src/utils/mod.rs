pub mod google;
pub mod twitter;
pub mod github;
pub mod stackoverflow;
pub mod reddit;
pub mod linkedin;

pub fn get_command_from_query_string(_query_string: &str)-> &str {
    if _query_string.contains(' ') {
        let _index_of_whitespace = _query_string.find(' ').unwrap_or(0);
        return &_query_string[.._index_of_whitespace];
    }
    return &_query_string;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
    // Test command with no whitespace
    let actual = get_command_from_query_string("tw");
    let expected = "tw";
    assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
    // Test command with whitespace
    let actual = get_command_from_query_string("tw @fbOpenSource");
    let expected = "tw";
    assert_eq!(actual, expected);
    }
}
