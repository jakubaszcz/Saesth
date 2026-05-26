use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{ActivityType, Assets, Button};
use crate::database::settings::database_settings::database_settings_get_active_setting;
use crate::global::global::PREFIX_FOR_SETTING;
use crate::types::settings::type_settings::SettingKeys;
use crate::utils::prefix::util_prefix::util_prefix_add_prefix;

const ID: &str = "1292058064416931963";

pub fn init() {
    if database_settings_get_active_setting(&*util_prefix_add_prefix(PREFIX_FOR_SETTING, &SettingKeys::DiscordRichPresence.to_key().as_str())) {
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
}