extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_reddit_url(_query: &str) -> String {
    if _query == "rd" {
        return String::from("https://reddit.com")
    } else if &_query[..5] == "rd r/" {
        return construct_reddit_profile_url(&_query[3..]);
    } else {
        return construct_reddit_search_url(&_query[3..]);
    }
}

pub fn construct_reddit_profile_url(_profile: &str) -> String {
    let _reddit_profile_url = format!("https://reddit.com/{}",_profile);
    return _reddit_profile_url;
}

pub fn construct_reddit_search_url(_query: &str) -> String {
    let encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _reddit_search_url = format!("https://reddit.com/search?q={}",encoded_query);
    return _reddit_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_reddit_url() {
        let _fake_query = "rd";
        assert_eq!(construct_reddit_url(_fake_query),
                    "https://reddit.com"
        );
    }

    #[test]
    fn test_construct_reddit_url_query() {
        let _fake_query = "rd hello world";
        assert_eq!(construct_reddit_url(_fake_query),
                    "https://reddit.com/search?q=hello%20world"
        );

    }

    #[test]
    fn test_construct_reddit_url_profile() {
        let _fake_profile = "rd r/SamayRaina";
        assert_eq!(construct_reddit_url(_fake_profile),
                    "https://reddit.com/r/SamayRaina"
        );

    }

    #[test]
    fn test_construct_reddit_profile_url() {
        let _fake_profile = "r/SamayRaina";
        assert_eq!(construct_reddit_profile_url(_fake_profile),
        "https://reddit.com/r/SamayRaina"
        );

    }


    #[test]
    fn test_construct_reddit_search_url() {
        let _fake_query = "hello world";
        assert_eq!(construct_reddit_search_url(_fake_query),
                    "https://reddit.com/search?q=hello%20world"
        );
    }

}
