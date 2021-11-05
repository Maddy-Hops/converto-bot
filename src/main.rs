use serenity::{
	async_trait,
	client::{Context, EventHandler},
	framework::{
		standard::{
			macros::{command, group},
			CommandResult,
		},
		StandardFramework,
	},
	model::{channel::Message, id::UserId},
	Client,
};
use std::{collections::HashSet, env};

mod commands;

use commands::conversion;

#[group]
#[commands(about)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if !msg.author.bot {
			if let Some(reply) = conversion::respond_to_msg(&msg.content){
				msg.reply(ctx, reply).await.unwrap();
			}
		}
	}
}

#[tokio::main]
async fn main() {
	let token: String = env::var("tutorial_bot_token").expect("Token not found");
	let mut owners = HashSet::new();
	owners.insert(UserId::from(360433679111159808));
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("!").owners(owners))
		.group(&GENERAL_GROUP);

	let mut client = Client::builder(token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Could not start Discord");

	client.start().await.expect("The bot stopped");
}


#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(
		ctx,
		"Maddy-hops' bot, is supposed to provide unit conversion services for my personal servers.",
	)
	.await?;
	Ok(())
}
