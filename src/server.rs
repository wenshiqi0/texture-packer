use std::thread;
use tiny_http::{Response, Server};

pub fn setup() {
    thread::spawn(|| match Server::http("0.0.0.0:9888") {
        Ok(server) => {
            for req in server.incoming_requests() {
                println!("{} {}", req.method(), req.remote_addr());
                let res = Response::from_string("hello world");
                req.respond(res);
            }
        }
        _ => (),
    });
}
