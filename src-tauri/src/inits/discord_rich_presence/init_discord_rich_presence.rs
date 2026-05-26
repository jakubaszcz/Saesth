use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{ActivityType, Assets, Button};

const ID: &str = "1292058064416931963";

pub fn init() {
    let Ok(mut client) = DiscordIpcClient::new(ID) else {
        return;
    };

    if client.connect().is_err() {
        return;
    }

    let _ = client.set_activity(
        activity::Activity::new()
            .activity_type(ActivityType::Listening)
            .state("Listening to ambient sound")
            .details("Work aesthetically")
    );
}