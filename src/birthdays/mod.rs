// module to keep track of user's birthdays through a connection to mongodb db
use serenity::framework::standard::{macros::command, CommandResult};



#[command]
#[owners_only]
pub async fn update_db() -> CommandResult{
	Ok(())
}