extern crate exitcode;
use std::env;
use std::process;
use serenity::{
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::Client,
};
#[group]
#[commands(ding, poop)]
struct General;

struct Handler;

impl EventHandler for Handler {
    // fn message(&self, ctx: Context, msg: Message) {
    //     match msg.content.as_ref() {
    //         "!ping" => send_message(&ctx, &msg, "Pong!"),
    //         "poop" => send_message(&ctx, &msg, ":poop:"),
    //         _ => (),
    //     }
    // }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
// fn send_message(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
//     if let Err(why) = msg.channel_id.say(&ctx.http, content) {
//         println!("Err msg: {:?}", why);
//     }
//}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No Token provided as argument 1");
        process::exit(exitcode::DATAERR);
    }
    let token = &args[1];

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}

#[command]
fn ding(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Skeet!")?;

    Ok(())
}

#[command]
fn poop(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, ":poop:")?;

    Ok(())
}