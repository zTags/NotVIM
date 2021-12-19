use discord_rich_presence::{activity, new_client, DiscordIpc};

pub fn start_rpc() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "id";
    let mut client = new_client(client_id)?;
    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("test")
        .details("test")
    )?;
    client.close()?;

    Ok(())
}