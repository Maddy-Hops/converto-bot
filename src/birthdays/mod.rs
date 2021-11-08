// module to keep track of user's birthdays through a connection to mongodb db
use serenity::{client::Context, framework::standard::{macros::command, CommandResult}, model::channel::Message, prelude::{RwLock, TypeMapKey}};
use mongodb::{options::ClientOptions};
use std::{collections::HashMap, env, sync::Arc};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};


pub struct BirthdaysDb;


impl TypeMapKey for BirthdaysDb {
    type Value = Arc<RwLock<HashMap<String, u64>>>;
}

#[derive(Debug, Serialize, Deserialize)]
struct Birthday {
    dob: String,
	discord_id: String,
}

#[command]
#[owners_only]
pub async fn update_db(_ctx: &Context, msg: &Message) -> CommandResult{
	let connection_string = env::var("db_connection_string").expect("Database connection string not found");
	let client_options = ClientOptions::parse(connection_string).await?;
	let client = mongodb::Client::with_options(client_options)?;
	let db = client.database("discord-bot");
	let birthdays = db.collection::<Birthday>("birthdays");
	let filter = doc! { "dob": "16/11" };
	let find_options = FindOptions::builder().sort(doc! { "discord_id": 1 }).build();
	let mut cursor = birthdays.find(filter,find_options).await?;
	while let Some(birthday) = &cursor.try_next().await?{
		println!("{:?}",birthday);
	}
	Ok(())
}

pub async fn notify_users(ctx: &Context, msg: &Message) {
	let channel = match msg.channel_id.to_channel(&ctx).await {
		Ok(channel) => channel,
		Err(why) => {
			println!("Error getting channel: {:?}", why);

			return;
		},
	};
	println!("We got it!!");

}