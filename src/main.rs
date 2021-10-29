use std::{collections::HashSet, env};
use serenity::{Client, async_trait, client::{Context, EventHandler}, framework::{StandardFramework, standard::{CommandResult, macros::{command, group}}}, model::{channel::Message, id::UserId}, prelude::*};

mod commands;

#[group]
#[commands(ping,about)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
	async fn message(&self, _ctx: Context, msg: Message) {
		if !msg.author.bot {
			println!("{:?}",msg.content);
		}
	}
}


#[tokio::main]
async fn main() {
	let token: String = env::var("tutorial_bot_token").expect("Token not found");
	let mut owners = HashSet::new();
	owners.insert(UserId::from(360433679111159808));
	let framework = StandardFramework::new()
		.configure(|c| {
			c.prefix("!").owners(owners)
		})
		.group(&GENERAL_GROUP);

	let mut client = Client::builder(token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Could not start Discord");

	client.start().await.expect("The bot stopped");
}



#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(ctx,"Pong!").await?;
	Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(ctx, "Maddy-hops' bot, is supposed to provide unit conversion services for my personal servers.").await?;
	Ok(())
}
