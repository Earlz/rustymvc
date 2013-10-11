use std::os::getenv;


fn main() {
    print("content-type: text/plain\r\n\r\n");
    print("hello from ");
    let path = match getenv("PATH_INFO") {
        Some(m) => m,
        None => ~"you didn't specify a path :("
    };
    println(path);
}