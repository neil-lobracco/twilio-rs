use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::net::SocketAddr;
use twilio::twiml::{Say, Twiml, Voice};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let app_id = "<app-id>";
    let auth_token = "<auth-token>";
    let client = twilio::Client::new(app_id, auth_token);

    let cloned_uri = req.uri().clone();
    println!("Got a request for: {}", cloned_uri);

    let response = match cloned_uri.path() {
        "/message" => {
            client
                .respond_to_webhook(req, |msg: twilio::Message| {
                    let mut t = Twiml::new();
                    t.add(&twilio::twiml::Message {
                        txt: format!("You told me: '{}'", msg.body.unwrap()),
                    });
                    t
                })
                .await
        }
        "/call" => {
            client
                .respond_to_webhook(req, |_: twilio::Call| {
                    let mut t = Twiml::new();
                    t.add(&Say {
                        txt: "Thanks for using twilio-rs. Bye!".to_string(),
                        voice: Voice::Woman,
                        language: "en".to_string(),
                    });
                    t
                })
                .await
        }
        _ => panic!("Hit an unknown path."),
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let make_service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = hyper::Server::bind(&addr).serve(make_service);
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}
