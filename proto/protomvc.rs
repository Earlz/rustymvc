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
    handler: ~fn(@mut HttpContext)
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
    fn execute(&self, c: @mut HttpContext) {
        for route in self.routes.iter(){
            if(route.path == c.request.path){
                (route.handler)(c);
            }
        }
    }
    fn controller<T>(@mut self, creator: ~fn(@mut HttpContext) -> T) -> ControllerBox<T>
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
    router: @mut Router,
    route: ~Route,
    creator: Option<~fn(@mut HttpContext) -> T>
}

impl<T> ControllerBox<T>{
    fn handles(@mut self, path: ~str) -> @mut ControllerBox<T>{
        self.route.path=path;
        self
    }
    fn with(@mut self, invoker: ~fn(&mut T)) -> @mut ControllerBox<T>{
        let tmp=self.creator.take();
        
        let h = |c| {
            match tmp {
                None => (),
                Some(ref t) => {
                    let mut ctrl=(*t)(c);
                    invoker(&mut ctrl);
                }
            };
        };
        self.router.add(~Route{path: self.route.path.clone(), handler:h});
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
    context: @mut HttpContext
}

impl TestController{
    fn new(c: @mut HttpContext) -> TestController{
        TestController{ context: c}
    }
    fn index(&mut self) {
        self.context.response.body.push_str("test index");
    }
}



fn default_handler(context: @mut HttpContext){
    context.response.body.push_str("404 not found");
}

fn index(context: @mut HttpContext){
    context.response.body.push_str("yay index");
}

fn foo(context: @mut HttpContext){
    context.response.body.push_str("yay foo");
}

fn main() {
    let mut context=@mut HttpContext::create();
    let mut router=@mut Router::new();
    router.add(~Route{path: ~"", handler: index});
    router.add(~Route{path: ~"/foo", handler: foo});

    let mut test = @mut router.controller(|c| TestController::new(c));
    test.handles(~"/test").with(|c| c.index());


    router.execute(context);



    println(context.response.contenttype);
    println("");
    println(context.response.body); 
}