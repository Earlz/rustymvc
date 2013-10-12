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
    fn execute(&self, req: &Request, res: &mut Response) {
        for route in self.routes.iter(){
            if(route.path == req.path){
                (route.handler)(req, res);
            }
        }
    }

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
    let mut routes=RouteList::new();
    routes.add(Route{path: ~"", handler: index});
    routes.add(Route{path: ~"/foo", handler: foo});
    routes.execute(&req, &mut res);
    println(res.contenttype);
    println("");
    println(res.body);
    println("hello from rust!");
    println!("path: {:s}", req.path);
    println!("querystring: {:s}", req.querystring);
    println("done!");
}