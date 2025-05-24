use serenity::all::{CreateEmbed, CreateEmbedFooter, EmbedFooter, Timestamp};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::commands::api::minecraft::mcapi_types::MCUser;
use crate::types::CommandReturn;

pub async fn run(options: &[ResolvedOption<'_>], api_client: &reqwest::Client) -> CommandReturn {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(username), ..
    }) = options.first() {
        let request_url = format!("https://playerdb.co/api/player/minecraft/{}", username);
        let response = api_client.get(request_url).header(reqwest::header::USER_AGENT, "jthomas@pixip.uk-discord-bot").send().await.expect("Unable to access api!");

        let mc_user: MCUser = response.json().await.expect("Unable to convert response to json!");
        if mc_user.success {
            let mc_player = mc_user.data.player.clone().unwrap();
            let mc_id = mc_player.id;
            let mc_avatar = mc_player.avatar;
            let time = chrono::Utc::now().to_rfc3339();
            let timestamp: Timestamp = time.parse().expect("Invalid Timestamp");
            let return_embed = CreateEmbed::new()
                            .color(0xa000ff)
                            .title(username.to_string())
                            .field("Minecraft ID", mc_id, true)
                            .thumbnail(mc_avatar)
                            .timestamp(timestamp)
                            .description("User information found!")
                            .footer(CreateEmbedFooter::new("User found!"));
            CommandReturn {
                embedded: Some(return_embed),
                message: None,
            }
        } else {
            let return_string = format!("Unable to find {} as a Minecraft username.", username);
            CommandReturn {
                embedded: None,
                message: Some(return_string),
            }
        }
    } else {
        CommandReturn {
            embedded: None,
            message: Some("Please provide a valid string".to_string()),
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("getmcid").description("Checks the us")
    .add_option(
        CreateCommandOption::new(CommandOptionType::String, "username", "The MC username")
        .required(true)
    )
}