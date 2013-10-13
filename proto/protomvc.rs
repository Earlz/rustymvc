use std::os::getenv;

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
    handler: ~fn(&mut HttpContext)
}

struct Router{
    routes: ~[Route],
}

impl Router{
    fn new() -> Router{
        Router{routes: ~[]}
    }
    fn add(&mut self, r: Route){
        self.routes.push(r);
    }
    fn execute(&self, c: &mut HttpContext) {
        for route in self.routes.iter(){
            if(route.path == c.request.path){
                (route.handler)(c);
            }
        }
    }
    fn controller<T>(~self, creator: ~fn(&HttpContext) -> T) -> ControllerBox<T>
    {
        ControllerBox{
            router: self,
            route: ~Route{path: ~"", handler: default_handler},
            creator: Some(creator)
        }
    }
}


trait HttpController{
    
}


struct ControllerBox<T>{
    router: ~Router,
    route: ~Route,
    creator: Option<~fn(&HttpContext) -> T>
}

impl<T> ControllerBox<T>{
    fn handles(@mut self, path: ~str) -> @mut ControllerBox<T>{
        self.route.path=path;
        self
    }
    fn with<'r>(&'r mut self, invoker: ~fn(&mut T)) -> &'r mut ControllerBox<T>{
        let tmp=self.creator.take();
        
        self.route.handler = |c| {
            match tmp {
                None => (),
                Some(ref t) => {
                    let mut ctrl=(*t)(c);
                    invoker(&mut ctrl);
                }
            };
        };
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

struct TestController{
    context: ~HttpContext
}

impl TestController{
    fn new(c: ~HttpContext) -> TestController{
        TestController{ context: c}
    }
    fn index(&mut self) {
        self.context.response.body.push_str("test index");
    }
}



fn default_handler(context: &mut HttpContext){
    context.response.body.push_str("404 not found");
}

fn index(context: &mut HttpContext){
    context.response.body.push_str("yay index");
}

fn foo(context: &mut HttpContext){
    context.response.body.push_str("yay foo");
}

fn main() {
    let mut context=HttpContext::create();
    let mut router=Router::new();
    router.add(Route{path: ~"", handler: index});
    router.add(Route{path: ~"/foo", handler: foo});

    let mut test = router.controller(|c| TestController::new(c));
    test.handles(~"/test").with(|c| c.index());


    router.execute(&mut context);



    println(context.response.contenttype);
    println("");
    println(context.response.body);
}