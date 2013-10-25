use simplepattern::ParameterDictionary;
use simplepattern::SimplePattern;
use simplepattern::PatternMatcher;

struct HttpContext{
    request: Request,
    response: Response,
}

impl HttpContext{
}

struct Request {
    path: ~str,
    querystring: ~str
}


struct Response{
    contenttype: ~str,
    body: ~str
}

impl Response{
    fn new() -> Response{
        Response{contenttype: ~"text/html", body: ~""}
    }
}


struct Route {
    matcher: ~PatternMatcher,
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
            let res=route.matcher.matches(c.request.path);
            
            if(res.is_match){
                (route.handler)(&mut ControllerContext{http: c, router: self, route_params: res.params});
                return;
            }
        }
        default_handler(&mut ControllerContext{http: c, router: self, route_params: ParameterDictionary::new()});
    }
    fn controller<'r,T>(&'r mut self, creator: ~fn(&mut ControllerContext) -> T) -> ControllerBox<'r,T>
    {
        ControllerBox{
            router: self,
            path: ~"",
            creator: Some(creator)
        }
    }
}


struct ControllerContext<'self>
{
    http: &'self mut HttpContext,
    router: &'self Router,
    route_params: ParameterDictionary
}


trait HttpController{
    
}


struct ControllerBox<'self,T>{
    router: &'self mut Router,
    path: ~str,
    creator: Option<~fn(&mut ControllerContext) -> T>
}

impl<'self,T> ControllerBox<'self,T>{
    fn handles(&'self mut self, path: ~str) -> &'self mut ControllerBox<'self,T>{
        self.path=path;
        self
    }
    fn with(&'self mut self, invoker: ~fn(&mut T, &mut ControllerContext)) -> &'self mut ControllerBox<'self,T>{
        let tmp=self.creator.take();
        
        self.router.add(~Route{matcher: ~SimplePattern::new(self.path) as ~PatternMatcher, handler:
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
/*
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
*/


fn default_handler(context: &mut ControllerContext){
    context.http.response.body.push_str("404 not found");
}
/*
fn main() {
    let mut context=HttpContext::create();
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
*/