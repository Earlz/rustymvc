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

#[test]
fn route_raw_text_doesnt_always_match_url() {
    let p=SimplePattern::new("/foo/[bar]/[biz=foo]");
    assert!(!p.matches("/foo/[bar]/[biz=foo]").is_match);
}