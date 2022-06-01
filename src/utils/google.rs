extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_google_search_from_query(_query : &str)-> String {
    let _encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _google_search_url = format!("https://google.com/search?q={}", _encoded_query);
    return _google_search_url;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let _fake_query = "hello";
        assert_eq!(construct_google_search_from_query(_fake_query),
                    "https://google.com/search?q=hello");
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let _fake_query = "hello world";
        assert_eq!(construct_google_search_from_query(_fake_query),
                    "https://google.com/search?q=hello%20world");
    }


}
