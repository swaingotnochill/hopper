extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_stackoverflow_url(_query: &str) -> String {
    if _query == "st" {
        return String::from("https://stackoverflow.com")
    } else {
        return construct_stackoverflow_search_url(&_query[3..]);
    }
}

pub fn construct_stackoverflow_search_url(_query: &str) -> String {
    let encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _stackoverflow_search_url = format!("https://stackoverflow.com/search?q={}",encoded_query);
    return _stackoverflow_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_stackoverflow_url() {
        let _fake_query = "st";
        assert_eq!(construct_stackoverflow_url(_fake_query),
                    "https://stackoverflow.com"
        );
    }

    #[test]
    fn test_construct_stackoverflow_url_query() {
        let _fake_query = "st hello world";
        assert_eq!(construct_stackoverflow_url(_fake_query),
                    "https://stackoverflow.com/search?q=hello%20world"
        );

    }

    #[test]
    fn test_construct_stackoverflow_search_url() {
        let _fake_query = "hello world";
        assert_eq!(construct_stackoverflow_search_url(_fake_query),
                    "https://stackoverflow.com/search?q=hello%20world"
        );
    }

}
