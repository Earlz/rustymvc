use std::os::getenv;
use std::vec::append_one;
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

struct Route {
    path: ~str,
    handler: ~fn(&Request)
}

struct RouteList{
    routes: ~[Route],
}

impl RouteList{
    fn new() -> RouteList{
        RouteList{routes: ~[]}
    }
    fn add(&mut self, r: Route){
        self.routes.push(r);
    }
    fn execute(&self, req: &Request) {
        for route in self.routes.iter(){
            if(route.path == req.path){
                (route.handler)(req);
            }
        }
    }

}


fn index(r: &Request){
    println("yay index");
}

fn foo(r: &Request){
    println("yay foo");
}

fn main() {
    print("content-type: text/plain\r\n\r\n");
    
    let req=Request::populate();
    let mut routes=RouteList::new();
    routes.add(Route{path: ~"", handler: index});
    routes.add(Route{path: ~"/foo", handler: foo});
    routes.execute(&req);
    println("hello from rust!");
    println!("path: {:s}", req.path);
    println!("querystring: {:s}", req.querystring);
    println("done!");
}