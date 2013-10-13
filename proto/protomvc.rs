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
    handler: ~fn(&Request, &mut Response)
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
    fn execute(&self, req: &Request, res: &mut Response) {
        for route in self.routes.iter(){
            if(route.path == req.path){
                (route.handler)(req, res);
            }
        }
    }
    fn controller<T>(@self, creator: ~fn(&HttpContext) -> T) -> ControllerBox<T>
    {
        ControllerBox{
            router: self,
            route: ~Route{path: ~"", handler: default_handler},
            creator: creator
        }
    }
}


trait HttpController{
    
}


struct ControllerBox<T>{
    router: @Router,
    route: ~Route,
    creator: ~fn(&HttpContext) -> T
}

impl<T> ControllerBox<T>{
    
}


fn default_handler(r: &Request, res: &mut Response){
    res.body.push_str("404 not found");
}

fn index(r: &Request,res: &mut Response){
    res.body.push_str("yay index");
}

fn foo(r: &Request, res: &mut Response){
    res.body.push_str("yay foo");
}

fn main() {
    let req=Request::populate();
    let mut res=Response::new();
    let mut router=Router::new();
    router.add(Route{path: ~"", handler: index});
    router.add(Route{path: ~"/foo", handler: foo});
    router.execute(&req, &mut res);
    println(res.contenttype);
    println("");
    println(res.body);
}