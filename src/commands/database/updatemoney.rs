use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use sqlx::pool::Pool;
use sqlx::{Executor, MySql};

pub async fn run(_options: &[ResolvedOption<'_>], database: &Pool<MySql>) -> String {
    
    let mut message = "Updated user money!".to_string();
    if let Err(_i) = database.execute("UPDATE users SET money=100 WHERE discordid=177497512561606656;").await {
        message = "Failed to update!".to_string();
    }
    message
}

pub fn register() -> CreateCommand {
    CreateCommand::new("updatemoney").description("Updates a user's money")
    .add_option(
        CreateCommandOption::new(CommandOptionType::User, "id", "The user to add")
        .required(true)
    )
}
