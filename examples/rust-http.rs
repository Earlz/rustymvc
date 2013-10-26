/*
This is a simple integration with rust-http. rust-http is still pretty rough around the edges,
so this sample is as well. 

To compile use something like so: (make sure you compile rust-http first)

    rustc -L ~/rust-http/build -L ../build/ rust-http.rs

To run, just do

    ./rust-http

You can test it out by going to http://127.0.0.1:8001/say/hello
*/
extern mod rustymvc;
extern mod extra;
extern mod http;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::arc::Arc;

use http::server::{Config, Server, ServerUtil, ResponseWriter};
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
    }
}

impl Server for MvcServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &http::server::Request, w: &mut ResponseWriter) {
        let mut c=context(r);
        let tmp = &mut self.router.clone();
        tmp.get().execute(&mut c);
        
        let parts=c.response.contenttype.split_iter('/').to_owned_vec();
        w.headers.content_type = Some(MediaType {
            type_: parts[0].to_owned(),
            subtype: parts[1].to_owned(),
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
            path: path_to_str(&req.request_uri),
            querystring: ~"" //currently rust-http doesn't parse query strings
        },
        response: Response::new()
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
