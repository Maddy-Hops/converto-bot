// module to keep track of user's birthdays through a connection to mongodb db
use serenity::{client::Context,model::channel::Message,framework::standard::{macros::command, CommandResult}};
use mongodb::{Client, options::ClientOptions};
use std::env;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};

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
	let client = Client::with_options(client_options)?;
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
