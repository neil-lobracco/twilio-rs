twilio-rs
=========
`twilio-rs` is a Rust library for integrating with Twilio. It tries to present an idiomatic Rust interface for making requests to the Twilio API, and validating, parsing, and replying to webhooks that Twilio makes to your server.

First, you'll need to create a Twilio client:

	let client = twilio::Client::new(ACCOUNT_ID,AUTH_TOKEN);
	
Now, you can use that client to make or receive Twilio requests. For example, to send a message:

	client.send_message(OutboundMessage::new(from,to,"Hello, World!"));

Or to make a call:

	client.make_call(OutboundCall::new(from,to,callback_url));
	
Of course, much of our interaction with Twilio is by defining resources that respond to Twilio webhooks. To respond to every SMS with a customized reply, in your server's handler method:

	fn handle_request(mut req: Request, res: Response)  {
		let client = ...;
		client.respond_to_webhook(&mut req, res, |msg: Message|{
			let mut t = Twiml::new();
            t.add(&twiml::Message {txt: format!("You told me: '{}'",msg.body.unwrap())});
            t
            });
		});
	}

Alternatively, to respond to a voice callback with a message:

	fn handle_request(mut req: Request, res: Response)  {
		let client = ...;
		client.respond_to_webhook(&mut req, res, |msg: Call|{
			let mut t = Twiml::new();
			t.add(&Say{txt: "Thanks for using twilio-rs. Bye!".to_string(),voice: Woman,language: "en".to_string()});
            t
            });
		});
	}

Using the `respond_to_webhook` method will first authenticate that the request came from Twilio, using your AuthToken. If that fails, an error will be sent to the client. Next, the call or message will be parsed from the parameters passed in. If a required field is missing, an error will be sent to the client. Finally, the parsed object will be passed to your handler method, which must return a `Twiml` that will be used to respond to the webhook.

The `respond_to_webhook` method is designed to work on [Hyper](https://github.com/hyperium/hyper) `Request`s and `Response`s. Hyper is also used internally to make requests to Twilio's API.