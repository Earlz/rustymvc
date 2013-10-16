use simplepattern;

use simplepattern::SimplePattern;


#[test]
fn root_url_matches_only_root() {
    let p=SimplePattern::new("/");
    assert!(p.matches("/").is_match);
/*
var x=new SimplePattern("/");
            Assert.IsTrue(x.Match("/").IsMatch);
            Assert.IsFalse(x.Match("/foo/bar").IsMatch);
            Assert.IsFalse(x.Match("/123").IsMatch);
*/
}
