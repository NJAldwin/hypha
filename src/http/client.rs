use http::request::Request;

pub fn execute(request: Request) {
    println!("Executing {:?}", request);
}
