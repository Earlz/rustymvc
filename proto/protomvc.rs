use std::os::getenv;
use std::rc::RcMut;

pub mod simplepattern;

struct HttpContext{
    request: Request,
    response: Response
}

impl HttpContext{
    fn create() -> HttpContext{
        HttpContext{
            request: Request::populate(),
            response: Response::new()
        }
    }
}

struct Request {
    path: ~str,
    querystring: ~str
}


impl Request {
    fn populate() -> Request {
        Request{
            path: getenv("PATH_INFO").unwrap_or(~""),
            querystring: getenv("QUERY_STRING").unwrap_or(~"")
        }
    }
}

struct Response{
    contenttype: ~str,
    body: ~str
}

impl Response{
    fn new() -> Response{
        Response{contenttype: ~"text/plain", body: ~""}
    }
}


struct Route {
    path: ~str,
    handler: ~fn(&mut ControllerContext)
}

struct Router{
    routes: ~[Route],
}


impl Router{
    fn new() -> Router{
        Router{routes: ~[]}
    }
    fn add(&mut self, r: ~Route){
        self.routes.push(*r);
    }
    fn execute(&mut self, c: &mut HttpContext) {
        for route in self.routes.iter(){
            if(route.path == c.request.path){
                (route.handler)(&mut ControllerContext{http: c, router: self});
            }
        }
    }
    fn controller<'r,T>(&'r mut self, creator: ~fn(&mut ControllerContext) -> T) -> ControllerBox<'r,T>
    {
        ControllerBox{
            router: self,
            route: ~Route{path: ~"", handler: default_handler},
            creator: Some(creator)
        }
    }
}


struct ControllerContext<'self>
{
    http: &'self mut HttpContext,
    router: &'self Router
}


trait HttpController{
    
}


struct ControllerBox<'self,T>{
    router: &'self mut Router,
    route: ~Route,
    creator: Option<~fn(&mut ControllerContext) -> T>
}

impl<'self,T> ControllerBox<'self,T>{
    fn handles(&'self mut self, path: ~str) -> &'self mut ControllerBox<'self,T>{
        self.route.path=path;
        self
    }
    fn with(&'self mut self, invoker: ~fn(&mut T, &mut ControllerContext)) -> &'self mut ControllerBox<'self,T>{
        let tmp=self.creator.take();
        
        self.router.add(~Route{path: self.route.path.clone(), handler:
         |c| {
            match tmp {
                None => (),
                Some(ref t) => {
                    let mut ctrl=(*t)(c);
                    invoker(&mut ctrl, c);
                }
            };
        }});
        self
    }
}
/*
struct Meh{
    biz: ~fn(int) -> int
}

impl Meh{
    fn foo(@mut self
}
*/

struct TestController;

impl TestController{
    fn new() -> TestController{
    TestController
    }
    fn index(&mut self, context: &mut ControllerContext) {
        context.http.response.body.push_str("test index");
    }
}



fn default_handler(context: &mut ControllerContext){
    context.http.response.body.push_str("404 not found");
}

fn index(context: &mut ControllerContext){
    context.http.response.body.push_str("yay index");
}

fn foo(context: &mut ControllerContext){
    context.http.response.body.push_str("yay foo");
}

fn main() {
    let mut context=HttpContext::create();
    let mut router=Router::new();
    router.add(~Route{path: ~"", handler: index});
    router.add(~Route{path: ~"/foo", handler: foo});
    {
        let mut test = router.controller(|_| TestController::new()); 
        test.handles(~"/test").with(|c,ctx| c.index(ctx));
    }

    router.execute(&mut context); //possible borrowing more than once



    println(context.response.contenttype);
    println("");
    println(context.response.body); 
}