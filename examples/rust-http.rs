//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.
extern mod rustymvc;
extern mod extra;
extern mod http;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;
use extra::arc::Arc;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::server::request::RequestUri;
use http::headers::content_type::MediaType;

use rustymvc::router::{ControllerContext, HttpContext, Router, Response, Request};


#[deriving(Clone)]
struct MvcServer{
    router: Arc<Router>
}

impl MvcServer {
    fn new() -> MvcServer {
        let router=MvcServer::init_router();
        
        MvcServer{router: Arc::new(router)}
    }
    fn init_router() -> Router {
        let mut router=Router::new();
        {
            let mut test = router.controller(|_| TestController::new()); 
            test.handles(~"/test").with(|c,ctx| c.index(ctx));
        }
        {
            let mut test = router.controller(|_| TestController::new()); 
            test.handles(~"/say/[message]").with(|c,ctx| c.say(ctx));
        }
        return router;
    //router.execute(&mut context); 
    }
}

impl Server for MvcServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &http::server::Request, w: &mut ResponseWriter) {
        let mut c=context(r);
        let mut tmp = &mut self.router.clone();
        tmp.get().execute(&mut c);
        /*
        w.headers.date = Some(time::now_utc());
        w.headers.content_length = Some(14);
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"plain",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        w.headers.server = Some(~"Example");
        */
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"plain",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        w.write(c.response.body.into_bytes());
    }

}



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

fn context(req: &http::server::Request) -> HttpContext {
    println!("uri: {:?}", req.request_uri);
    HttpContext {
        request: Request{
            path: path_to_str(&req.request_uri),//format!("{:?}", req.request_uri),
            querystring: ~""
        },
        response: Response::new()
    }
}

enum Foo
{
    Star,
    Baz(Router),
    Bar(~str)
}
fn tester(foo: Foo) -> ~str{
    match foo {
        Star => ~"",
        Bar(x) => x.to_owned(),
        Baz(r) => ~"foo"
    }
}

fn path_to_str(r: &http::server::request::RequestUri) -> ~str {
    match *r {
        http::server::request::Star => ~"",
        http::server::request::AbsoluteUri(_) => ~"",
        http::server::request::AbsolutePath(ref path) => path.to_owned(),
        http::server::request::Authority(_) => ~""
    }
    
}

fn main() {
    MvcServer::new().serve_forever();
}
