//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.
extern mod rustymvc;
extern mod extra;
extern mod http;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;
use std::rc::RcMut;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use rustymvc::router::{ControllerContext, HttpContext, Router, Response, Request};


#[deriving(Clone)]
struct MvcServer{
    router: RcMut<Router>
}

impl MvcServer {
    fn new() -> MvcServer {
        let router=MvcServer::init_router();
        
        MvcServer{router: RcMut::new(router)}
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

    fn handle_request(&self, _r: &http::server::Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_length = Some(14);
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"plain",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        w.headers.server = Some(~"Example");

        w.write(bytes!("Hello, World!\n"));
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

fn context(req: http::server::Request) -> HttpContext {
    HttpContext {
        request: Request{
            path: format!("{:?}", req.request_uri),
            querystring: ~""
        },
        response: Response::new()
    }
}
/*
fn path_to_str(uri: RequestUri) -> ~str {
    match(uri) {
        Star => ~"",
        AbsoluteUri(path) => ~"",
        AbsolutePath(path) => ~path.clone(),
        Authority => ~""
    }
}
*/
fn main() {
    let server = MvcServer::new();;
    server.serve_forever();
}
