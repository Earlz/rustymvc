/* 
This is a simple executable capable of being run as a cgi-bin executable. 
To compile, use something like 
    rustc -L ../build/ cgibin.rs

To run with a simple server like thttpd, use something like so:
#thttpd.conf
port=8080
cgipat=**
#to actually run:
thttpd -C thttpd.conf -D

And test it out by browsing to http://127.0.0.1:8080/cgibin/say/hello
*/


extern mod rustymvc;

use std::os::getenv;
use rustymvc::router::{ControllerContext, HttpContext, Router, Response, Request};

struct TestController;

impl TestController{
    fn new() -> TestController{
    TestController
    }
    fn index(&mut self, context: &mut ControllerContext) {
        context.http.response.body.push_str("test index");
    }
    fn say(&mut self, context: &mut ControllerContext) {
        context.http.response.body.push_str(context.route_params[~"message"]);
    }
}

fn context() -> HttpContext {
    HttpContext {
        request: Request{
            path: getenv("PATH_INFO").unwrap_or(~""),
            querystring: getenv("QUERY_STRING").unwrap_or(~"")
        },
        response: Response::new()
    }
}

fn main() {
    let mut context=context();
    let mut router=Router::new();
    {
        let mut test = router.controller(|_| TestController::new()); 
        test.handles(~"/test").with(|c,ctx| c.index(ctx));
    }
    {
        let mut test = router.controller(|_| TestController::new()); 
        test.handles(~"/say/[message]").with(|c,ctx| c.say(ctx));
    }

    router.execute(&mut context); 



    println(context.response.contenttype);
    println("");
    println(context.response.body); 
}