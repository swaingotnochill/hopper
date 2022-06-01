extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_twitter_url(_query: &str) -> String {
    if _query == "tw" {
        return String::from("https://twitter.com")
    } else if &_query[..4] == "tw @" {
        return construct_twitter_profile_url(&_query[4..]);
    } else {
        return construct_twitter_search_url(&_query[3..]);
    }
}

pub fn construct_twitter_profile_url(_profile: &str) -> String {
    let _twitter_profile_url = format!("https://twitter.com/{}",_profile);
    return _twitter_profile_url;
}

pub fn construct_twitter_search_url(_query: &str) -> String {
    let encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _twitter_search_url = format!("https://twitter.com/search?q={}",encoded_query);
    return _twitter_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let _fake_query = "tw";
        assert_eq!(construct_twitter_url(_fake_query),
                    "https://twitter.com"
        );
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let _fake_query = "tw hello world";
        assert_eq!(construct_twitter_url(_fake_query),
                    "https://twitter.com/search?q=hello%20world"
        );

    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let _fake_profile = "tw @_SnRoshan";
        assert_eq!(construct_twitter_url(_fake_profile),
                    "https://twitter.com/_SnRoshan"
        );

    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let _fake_profile = "_SnRoshan";
        assert_eq!(construct_twitter_profile_url(_fake_profile),
        "https://twitter.com/_SnRoshan"
        );

    }


    #[test]
    fn test_construct_twitter_search_url() {
        let _fake_query = "hello world";
        assert_eq!(construct_twitter_search_url(_fake_query),
                    "https://twitter.com/search?q=hello%20world"
        );
    }

}
