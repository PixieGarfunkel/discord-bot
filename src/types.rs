use serenity::all::CreateEmbed;

pub struct CommandReturn {
    pub message: Option<String>,
    pub embedded: Option<CreateEmbed>,
}