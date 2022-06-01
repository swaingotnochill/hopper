extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_github_url(_query: &str) -> String {
    if _query == "gh" {
        return String::from("https://github.com")
    } else if &_query[..4] == "gh @" {
        return construct_github_profile_url(&_query[4..]);
    } else if &_query[..4] == "gh /" {
        return construct_github_profile_url(&_query[4..]);
    } else {
        return construct_github_search_url(&_query[3..]);
    }
}

pub fn construct_github_profile_url(_profile: &str) -> String {
    let _github_profile_url = format!("https://github.com/{}",_profile);
    return _github_profile_url;
}

pub fn construct_github_search_url(_query: &str) -> String {
    let encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _github_search_url = format!("https://github.com/search?q={}",encoded_query);
    return _github_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_url() {
        let _fake_query = "gh";
        assert_eq!(construct_github_url(_fake_query),
                    "https://github.com"
        );
    }

    #[test]
    fn test_construct_github_url_query() {
        let _fake_query = "gh hello world";
        assert_eq!(construct_github_url(_fake_query),
                    "https://github.com/search?q=hello%20world"
        );

    }

    #[test]
    fn test_construct_github_url_profile() {
        let _fake_profile = "gh @swaingotnochill";
        assert_eq!(construct_github_url(_fake_profile),
                    "https://github.com/swaingotnochill"
        );

    }

    #[test]
    fn test_construct_github_profile_url() {
        let _fake_profile = "_SnRoshan";
        assert_eq!(construct_github_profile_url(_fake_profile),
        "https://github.com/_SnRoshan"
        );

    }


    #[test]
    fn test_construct_github_search_url() {
        let _fake_query = "hello world";
        assert_eq!(construct_github_search_url(_fake_query),
                    "https://github.com/search?q=hello%20world"
        );
    }

  #[test]
    fn test_construct_github_repo_url() {
        let _fake_query = "gh /swaingotnochill/swaingotnochill";
        assert_eq!(construct_github_url(_fake_query),
                    "https://github.com/swaingotnochill/swaingotnochill"
        );
    }



}
