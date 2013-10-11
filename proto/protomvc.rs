use std::os::getenv;

struct Request {
    path: ~str,
    querystring: ~str
}


fn main() {
    print("content-type: text/plain\r\n\r\n");
    println("hello from rust!");
    let req=Request{ path: getenv("PATH_INFO").unwrap_or(~""), querystring: getenv("QUERY_STRING").unwrap_or(~"")};
    println!("path: {:s}", req.path);
    println!("querystring: {:s}", req.querystring);
    println("done!");
}