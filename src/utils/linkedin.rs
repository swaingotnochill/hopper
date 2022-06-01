extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
                            .add(b'>').add(b'`');

pub fn construct_linkedin_url(_query: &str) -> String {
    if _query == "ln" {
        return String::from("https://linkedin.com")
    } else if &_query[..4] == "ln @" {
        return construct_linkedin_profile_url(&_query[4..]);
    } else {
        return construct_linkedin_search_url(&_query[3..]);
    }
}

pub fn construct_linkedin_profile_url(_profile: &str) -> String {
    let _linkedin_profile_url = format!("https://linkedin.com/in/{}",_profile);
    return _linkedin_profile_url;
}

pub fn construct_linkedin_search_url(_query: &str) -> String {
    let encoded_query = utf8_percent_encode(_query, FRAGMENT);
    let _linkedin_search_url = format!("https://linkedin.com/search/results/all/?keywords={}",encoded_query);
    return _linkedin_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_linkedin_url() {
        let _fake_query = "ln";
        assert_eq!(construct_linkedin_url(_fake_query),
                    "https://linkedin.com"
        );
    }

    #[test]
    fn test_construct_linkedin_url_query() {
        let _fake_query = "ln hello world";
        assert_eq!(construct_linkedin_url(_fake_query),
                    "https://linkedin.com/search/results/all/?keywords=hello%20world"
        );

    }

    #[test]
    fn test_construct_linkedin_url_profile() {
        let _fake_profile = "ln @snroshan";
        assert_eq!(construct_linkedin_url(_fake_profile),
                    "https://linkedin.com/in/snroshan"
        );

    }

    #[test]
    fn test_construct_linkedin_profile_url() {
        let _fake_profile = "snroshan";
        assert_eq!(construct_linkedin_profile_url(_fake_profile),
        "https://linkedin.com/in/snroshan"
        );

    }


    #[test]
    fn test_construct_linkedin_search_url() {
        let _fake_query = "hello world";
        assert_eq!(construct_linkedin_search_url(_fake_query),
                    "https://linkedin.com/search/results/all/?keywords=hello%20world"
        );
    }

}
