use chrono::{Date, Datelike, NaiveDate, Utc};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};
use serenity::{
	client::Context,
	framework::standard::{macros::command, Args, CommandResult, Delimiter},
	model::channel::Message,
	prelude::{RwLock, TypeMapKey},
};
use std::{collections::HashMap, env, sync::Arc};

use crate::TodayDate;

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
pub async fn update_db(ctx: &Context, _msg: &Message) -> CommandResult {
	database_update(ctx).await?;
	// set flag date to trigger search for birthday boys and girls again
	{
		let date_lock = {
			let data_write = ctx.data.read().await;
			data_write
				.get::<TodayDate>()
				.expect("Expected a TodayDate")
				.clone()
		};

		{
			let mut flag_date_write = date_lock.write().await;
			*flag_date_write = Date::<Utc>::from_utc(NaiveDate::from_yo(2021, 1), Utc);
		}
	}
	Ok(())
}
#[command]
#[owners_only]
pub async fn add_birthday(ctx: &Context, msg: &Message) -> CommandResult {
	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();
	if let Ok(discord_id) = args.parse::<String>() {
		args.advance();
		if let Ok(dob) = args.parse::<String>() {
			let connection_string = env::var("DB_CONNECTION_STRING").expect("Database connection string not found");
			let data_for_insertion = Birthday { dob, discord_id };
			{
				let client = mongodb::Client::with_uri_str(connection_string).await?;
				let db = client.database("discord-bot");
				let birthdays = db.collection::<Birthday>("birthdays");
				birthdays
					.insert_one(data_for_insertion, None)
					.await
					.unwrap();
			}
		} else {
			msg.reply(&ctx.http, "Need to specify dob for adding an entry")
				.await?;
		}
	} else {
		msg.reply(&ctx.http, "Need to specify discord_id for adding an entry")
			.await?;
	}
	Ok(())
}

#[command]
#[owners_only]
pub async fn delete_birthday(ctx: &Context, msg: &Message) -> CommandResult {
	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();
	if let Ok(query) = args.parse::<String>() {
		let connection_string = env::var("DB_CONNECTION_STRING").expect("Database connection string not found");

		{
			let client = mongodb::Client::with_uri_str(connection_string).await?;
			let db = client.database("discord-bot");
			let birthdays = db.collection::<Birthday>("birthdays");
			match birthdays
				.delete_many(doc! {"discord_id": query}, None)
				.await
			{
				Ok(deleted_entries) => {
					msg.reply(
						&ctx.http,
						format!(
							"Deleted {} entries matching your query",
							deleted_entries.deleted_count
						),
					)
					.await?;
				}
				Err(why) => {
					msg.reply(
						&ctx.http,
						format!("There was an error processing your querry:\n{:?}", why),
					)
					.await?;
				}
			}
		}
	} else {
		msg.reply(
			&ctx.http,
			"Need to specify discord_id for deleting an entry",
		)
		.await?;
	}
	Ok(())
}

#[command]
#[owners_only]
pub async fn print_db(ctx: &Context, _msg: &Message) -> CommandResult {
	let birthdays_local = {
		let data_read = ctx.data.read().await;
		data_read
			.get::<BirthdaysDb>()
			.expect("Expected a BirthdaysDb")
			.clone()
			.read()
			.await
			.clone()
	};
	println!("{:?}", birthdays_local);
	Ok(())
}

pub async fn database_update(ctx: &Context) -> CommandResult {
	let connection_string = env::var("DB_CONNECTION_STRING").expect("Database connection string not found");
	let mut birthdays_dict: HashMap<String, u64> = HashMap::new();
	{
		let client = mongodb::Client::with_uri_str(connection_string).await?;
		let db = client.database("discord-bot");
		let birthdays = db.collection::<Birthday>("birthdays");
		let filter = doc! {};
		let find_options = FindOptions::builder()
			.sort(doc! { "discord_id": 1_i32 })
			.build();
		let mut cursor = birthdays.find(filter, find_options).await?;

		// fill local dictionary from birthdayDb
		while let Some(birthday) = &cursor.try_next().await? {
			birthdays_dict.insert(birthday.dob.clone(), birthday.discord_id.parse().unwrap());
		}
	}
	let data_lock = {
		let data_write = ctx.data.read().await;
		data_write
			.get::<BirthdaysDb>()
			.expect("Expected a BirthdaysDb")
			.clone()
	};
	// read all birthdays to memory
	{
		let mut birthdays_db = data_lock.write().await;
		*birthdays_db = birthdays_dict;
	}
	Ok(())
}

pub async fn notify_users(ctx: &Context, msg: &Message) {
	let channel = std::env::var("GENERAL_CHANNEL")
		.expect("Failed to lookup general-channel id")
		.parse::<u64>()
		.unwrap();

	let mut birthdays = {
		let data_read = ctx.data.read().await;
		let birthdays_lock = data_read
			.get::<BirthdaysDb>()
			.expect("expected a TodayDate")
			.clone();
		let birthdays = birthdays_lock.read().await;
		birthdays.clone()
	};
	println!("{:?}", birthdays);

	let query = {
		let date = msg.timestamp.date();
		format!("{}/{}", date.day(), date.month())
	};

	birthdays.retain(|dob, _| dob == &query);
	if !birthdays.is_empty() {
		let mut message = String::new();
		message += &format!("Today's ({}) the birthday of:", msg.timestamp.date());
		for (_, id) in birthdays {
			message += &format!("\n<@!{}>", id);
		}
		message += "\n🎂HAPPY BIRTHDAY TO THEM🎂";
		let channel = ctx
			.cache
			.guild_channel(channel)
			.await
			.expect("Channel with that ID isnt found");
		channel
			.say(&ctx.http, message)
			.await
			.expect("Failed to send message");
	}
}
