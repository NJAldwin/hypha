use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use http::request::Request;

// TODO @NJA: custom error type
pub fn execute(request: Request) -> Result<String, io::Error> {
    println!("Executing {:?}", &request);

    let mut stream = try!(TcpStream::connect(&request.url));

    // TODO @NJA: generate this separately, support headers, etc
    let _ = try!(write!(stream,
                        "{} {} {}\r\n\
                         Host: {}\r\n\
                         Connection: close\r\n\r\n",
                        &request.method, &request.url.path(), &request.version,
                        &request.url.host().unwrap()));

    let mut buffer = String::new();

    let _ = try!(stream.read_to_string(&mut buffer));

    Ok(buffer)
}
