use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::commands::api::minecraft::mcapi_types::MCUser;

pub async fn run(options: &[ResolvedOption<'_>], api_client: &reqwest::Client) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(username), ..
    }) = options.first() {
        let request_url = format!("https://playerdb.co/api/player/minecraft/{}", username);
        let response = api_client.get(request_url).header(reqwest::header::USER_AGENT, "jthomas@pixip.uk-discord-bot").send().await.expect("Unable to access api!");

        let mc_user: MCUser = response.json().await.expect("Unable to convert response to json!");
        if mc_user.success {
            let return_username = format!("Here's {}'s Minecraft user id: {}", username, mc_user.data.player.unwrap().id);
            return_username
        } else {
            let return_string = format!("Unable to find {} as a Minecraft username.", username);
            return_string
        }
    } else {
        "Please provide a valid string".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("getmcid").description("Checks the us")
    .add_option(
        CreateCommandOption::new(CommandOptionType::String, "username", "The MC username")
        .required(true)
    )
}