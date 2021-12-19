use discord_rich_presence::{activity, new_client, DiscordIpc};

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    // TODO implement this
    let client_id = "insert client id when ready";
    let mut rpc = new_client(client_id)?;
}