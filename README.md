# RustyMVC

RustyMVC is a portable MVC framework for Rust.

# Goals

1. Portable to different HTTP server interfaces (currently cgi-bin and rust-http)
2. Embrace static typing, not work around it. As little reflection as possible

RustyMVC is based off of [LucidMVC](https://github.com/Earlz/lucidmvc), an MVC framework with similar goals, but written in .NET. 
Although it is not intended to be source compatible in any way, most design decisions are influenced by my existing code there. 

# Example usage

Example usage varies because of portabiility to different HTTP servers, but here is an illustrative example:


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
    
    //In the server setup code:
    let mut router=Router::new();
    {
        let mut test = router.controller(|_| TestController::new()); 
        test.handles(~"/test").with(|c,ctx| c.index(ctx));
    }
    {
        let mut test = router.controller(|_| TestController::new()); 
        test.handles(~"/say/[message]").with(|c,ctx| c.say(ctx));
    }
    
    //Executed upon each request
    let mut context=context();
    router.execute(&mut context); 

It's still rough around the edges, but I'm still learning Rust, so it'll get better :)
    

# Licensing

It is BSD licensed. I will not accept pull requests for LGPL or GPL licensed code. Other permissive licenses(MIT, etc) are fine though.