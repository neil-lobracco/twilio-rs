twilio-rs
=========
[![](http://meritbadge.herokuapp.com/twilio)](https://crates.io/crates/twilio)


`twilio-rs` is a Rust library for integrating with Twilio. It tries to present an idiomatic Rust interface for making requests to the Twilio API, and validating, parsing, and replying to webhooks that Twilio makes to your server.

First, you'll need to create a Twilio client:

```rust
let client = twilio::Client::new(ACCOUNT_ID, AUTH_TOKEN);
```
	
Now, you can use that client to make or receive Twilio requests. For example, to send a message:

```rust
client.send_message(OutboundMessage::new(from, to, "Hello, World!")).await;
```

Or to make a call:

```rust
client.make_call(OutboundCall::new(from, to, callback_url)).await;
```
	
Of course, much of our interaction with Twilio is by defining resources that respond to Twilio webhooks. To respond to every SMS with a customized reply, in your server's handler method:

```rust
use hyper::{Body, Request, Response};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = ...;
    let response = client.respond_to_webhook(req, |msg: Message| {
        let mut t = Twiml::new();
        t.add(&twiml::Message {
            txt: format!("You told me: '{}'",
            msg.body.unwrap()),
        });
        t
    })
    .await;
    Ok(response)
}
```

Alternatively, to respond to a voice callback with a message:

```rust
use hyper::{Body, Request, Response};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = ...;
    let response = client.respond_to_webhook(req, |msg: Call| {
        let mut t = Twiml::new();
        t.add(&twitml::Say {
            txt: "Thanks for using twilio-rs. Bye!".to_string(),
            voice: Voice::Woman,
            language: "en".to_string(),
        });
        t
    })
    .await;
    Ok(response)
}
```

Using the `respond_to_webhook` method will first authenticate that the request came from Twilio, using your AuthToken. If that fails, an error will be sent to the client. Next, the call or message will be parsed from the parameters passed in. If a required field is missing, an error will be sent to the client. Finally, the parsed object will be passed to your handler method, which must return a `Twiml` that will be used to respond to the webhook.

The `respond_to_webhook` method is designed to work on [Hyper](https://github.com/hyperium/hyper) `Request`s and `Response`s. Hyper is also used internally to make requests to Twilio's API.
