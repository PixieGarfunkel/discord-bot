use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use sqlx::pool::Pool;
use sqlx::MySql;
use sqlx::query;
use sqlx::error::Error;

use log;

pub async fn run(options: &[ResolvedOption<'_>], database: &Pool<MySql>) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first()
    {
        let user_data_result = query("SELECT * FROM users WHERE discordid=?;")
                    .bind(user.id.get())
                    .fetch_one(database).await;
        if let Err(why) = user_data_result {
            println!("Database Error: {why:?}");
            let message = match why {
                Error::RowNotFound => {
                    if let Err(why) = query("INSERT INTO users (discordid) VALUES (?);")
                        .bind(user.id.get())
                        .execute(database)
                        .await
                    {
                        log::warn!(target: "database_events", "Database Error: {why:?}");
                        format!("Sorry {}, I'm unable to add you to the database...", user.tag())
                    } else {
                        log::info!(target: "database_events", "Added {} (id: {}) to the database.", user.tag(), user.id.to_string());
                        format!("Welcome {}, I've added you to the database!", user.tag())
                    }
                }

                _ => {
                    log::warn!(target: "database_events", "Database Error: {why:?}");
                    format!("Sorry {}, I'm unable to check if you're in the database...", user.tag())
                }
            };
            message
        } else {
            format!("Welcome back {}, you're already in the database!", user.tag())
        }
    } else {
        "Please provide a valid user".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("adduser").description("Add a user to the database").add_option(
        CreateCommandOption::new(CommandOptionType::User, "id", "The user to add")
            .required(true),
    )
}
