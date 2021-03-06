use chrono::{Date, NaiveDate, Utc};
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
	model::{channel::Message, id::UserId, prelude::Ready},
	prelude::{RwLock, TypeMapKey},
	Client,
};

use std::{
	collections::{HashMap, HashSet},
	env,
	sync::Arc,
};

mod birthdays;
mod conversion;

use birthdays::*;

struct TodayDate;
impl TypeMapKey for TodayDate {
	type Value = Arc<RwLock<Date<Utc>>>;
}

#[group]
#[commands(about, update_db, add_birthday, print_db, delete_birthday)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		// check if we've sent a birthday reminder today (if a flag_date has already been set to today's date then we have!)
		let date = msg.timestamp.date();
		let flag_date = {
			let data_read = ctx.data.read().await;
			let today_date_lock = data_read
				.get::<TodayDate>()
				.expect("expected a TodayDate")
				.clone();
			let today_date = today_date_lock.read().await;
			today_date.clone()
		};
		if date != flag_date {
			update_flag(&ctx, date)
				.await
				.expect("failed to update the date in the DB");
			notify_users(&ctx, &msg).await;
		}
		if !msg.author.bot {
			if let Some(reply) = conversion::respond_to_msg(&msg.content) {
				msg.reply(&ctx, reply).await.unwrap();
				println!("{:?}", msg.timestamp.date())
			}
		}
	}
	async fn ready(&self, ctx: Context, _ready: Ready) {
		database_update(&ctx)
			.await
			.expect("failed to update database");
	}
}

#[tokio::main]
async fn main() {
	let token: String = env::var("DISCORD_API_TOKEN").expect("Token not found");

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

	// creating empty globals
	{
		let mut data = client.data.write().await;
		data.insert::<BirthdaysDb>(Arc::new(RwLock::new(HashMap::default())));
		data.insert::<TodayDate>(Arc::new(RwLock::new(Date::<Utc>::from_utc(
			NaiveDate::from_yo(2021, 1),
			Utc,
		))));
	}
	client.start().await.expect("The bot stopped");
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(
		ctx,
		"I provide unit conversion capabilities! (and also track birthdays!)",
	)
	.await?;
	Ok(())
}
