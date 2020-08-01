extern crate exitcode;
use std::env;
use std::process;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::Client,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_ref() {
            "!ping" => send_message(&ctx, &msg, "Second Pong"),
            _ => (),
        }
        // if msg.content == "!ping" {
        //     if let Err(why) = msg.channel_id.say(&ctx.http, "!ping") {
        //         println!("Error sending message: {:?}", why);
        //     }
        // }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
fn send_message(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content) {
        println!("Err msg: {:?}", why);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No Token provided as argument 1");
        process::exit(exitcode::DATAERR);
    }
    let token = &args[1];

    let mut client = Client::new(&token, Handler).expect("Error creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}
