extern crate http;
extern crate iron;
extern crate router;

use std::io::net::ip::{Ipv4Addr, Port};
use std::str;

use self::router::{Router, Params};
use self::iron::{Iron, Request, Response, IronResult, Set};
use self::iron::response::modifiers::{Status, Body};
use self::iron::status;

use serialize::json;

use commands::{PeekCommand, PushCommand, ClearCommand};

/// Http Server to make pushes, peeks and clears
pub struct Server {
    port: Port,
}

impl Server {
    /// Creates new instance of server
    pub fn new(port: Port) -> Server {
        Server {
            port: port,
        }
    }

    /// Starts listening server on specified port
    pub fn start(&mut self) {
        let mut router = Router::new();

        router.get("/hello/:name", Server::hello);
        router.get("/peek/:river", Server::peek);
        router.get("/peek/:river/:offset", Server::peek);
        router.post("/push/:river", Server::push);
        router.delete("/clear/:river", Server::clear);

        Iron::new(router).listen(Ipv4Addr(0, 0, 0, 0), self.port);
    }

    fn hello(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.get::< Router, Params >().unwrap();
        let name = params.find("name").unwrap();

        Ok(Response::new().set(Status(status::Ok)).set(Body(format!("Hello, {}!", name))))
    }

    fn peek(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.get::< Router, Params >().unwrap();
        let river = params.find("river").unwrap();
        let offset = from_str::< uint >(params.find("offset").unwrap_or(""));

        match PeekCommand::new().execute(river, offset) {
            Some(result) => Ok(Response::new().set(Status(status::Ok)).set(Body(json::encode(&result)))),
            _ => Ok(Response::new().set(Status(status::NotFound)).set(Body("NotFound")))
        }
    }

    fn push(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.get::< Router, Params >().unwrap();
        let river = params.find("river").unwrap();
        let message = str::from_utf8(req.body.as_slice());

        match message {
            Some(message) => {
                PushCommand::new().execute(river, message);
                Ok(Response::new().set(Status(status::Created)).set(Body("OK")))
            },
            None => Ok(Response::new().set(Status(status::BadRequest)).set(Body("unable to parse response body as utf8")))
        }
    }

    fn clear(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.get::< Router, Params >().unwrap();
        let river = params.find("river").unwrap();

        ClearCommand::new().execute(river);
        Ok(Response::new().set(Status(status::Ok)).set(Body("OK")))
    }
}
