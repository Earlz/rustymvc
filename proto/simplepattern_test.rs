use simplepattern;

use simplepattern::SimplePattern;


#[test]
fn root_url_matches_only_root() {
    let p=SimplePattern::new("/");
    assert!(p.matches("/").is_match);
}

#[test]
fn raw_routes_match_urls() {
    let p=SimplePattern::new("/foo/bar");
    assert!(p.matches("/foo/bar").is_match);
    assert!(p.matches("/foo/bar/").is_match);
    assert!(!p.matches("/biz/bar").is_match);
    assert!(!p.matches("/foo/bar/baz").is_match);
    assert!(!p.matches("/").is_match);
}
