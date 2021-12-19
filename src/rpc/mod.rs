use discord_rich_presence::{activity, new_client, DiscordIpc};
use std::thread;

pub fn start_rpc() {
    thread::spawn(|| {
        let version: &str = env!("CARGO_PKG_VERSION");
        let client_id = "client id";
        let mut client = new_client(client_id).unwrap();
        client.connect().unwrap();
        client.set_activity(activity::Activity::new()
            .state("Not editing")
            .details("Idle")
            .assets(activity::Assets::new()
                .large_image("idle")
                .large_text("Idle")
                .small_image("logo")
                .small_text(&format!("NotVIM v{}", version))
            )
        ).unwrap();

        loop {}
    });
}

pub fn stop_rpc() {
    // TODO implement
}