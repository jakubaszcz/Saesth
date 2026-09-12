use std::{
    sync::{atomic::Ordering, Once},
    thread,
    time::{Duration, Instant},
};
use discord_rich_presence::{
    activity::{Activity, ActivityType, Button},
    DiscordIpc, DiscordIpcClient,
};
use crate::{
    global::global::{PACK, SOUNDS},
    inits::manifest::init_manifest::setting_active,
    types::settings::type_settings::SettingKeys,
};

const APPLICATION_ID: &str = "1292058064416931963";
const WEBSITE_URL: &str = "https://www.saesth.com";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const RETRY_INTERVAL: Duration = Duration::from_secs(10);
const REFRESH_INTERVAL: Duration = Duration::from_secs(30);
static START: Once = Once::new();

#[derive(Debug, PartialEq, Eq)]
struct Presence {
    details: String,
    state: String,
}

impl Presence {
    fn new(pack: Option<&str>, playing: &[String]) -> Self {
        match pack {
            None => Self {
                details: "Finding a little atmosphere".into(),
                state: "Browsing sound packs".into(),
            },
            Some(name) => Self {
                details: short_text(&format!("Pack: {name}")),
                state: if playing.is_empty() {
                    "Taking a quiet break".into()
                } else {
                    short_text(&format!("Listening to {}", playing.join(" + ")))
                },
            },
        }
    }
}

fn short_text(text: &str) -> String {
    let clean = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.len() <= 120 { return clean; }
    let mut end = 117;
    while !clean.is_char_boundary(end) { end -= 1; }
    format!("{}?", &clean[..end])
}

fn current_presence() -> Presence {
    let name = PACK.get().and_then(|pack| {
        let pack = pack.lock().ok()?;
        (!pack.id.is_empty()).then(|| pack.name.clone())
    });
    let playing = SOUNDS.get().and_then(|sounds| sounds.lock().ok()).map(|sounds| {
        sounds.iter().filter(|sound| sound.play.load(Ordering::Relaxed))
            .map(|sound| sound.sound_id.replace('_', " "))
            .collect::<Vec<_>>()
    }).unwrap_or_default();
    Presence::new(name.as_deref(), &playing)
}

pub fn init() {
    START.call_once(|| { thread::spawn(run); });
}

fn run() {
    let setting = SettingKeys::DiscordRichPresence.to_key();
    let mut client: Option<DiscordIpcClient> = None;
    let mut last_presence = None;
    let mut last_sent = Instant::now();
    let mut retry_at = Instant::now();

    loop {
        if !setting_active(&setting) {
            if let Some(mut connected) = client.take() {
                let _ = connected.clear_activity();
                let _ = connected.close();
            }
            last_presence = None;
            retry_at = Instant::now();
        } else if Instant::now() >= retry_at {
            if client.is_none() {
                let mut candidate = DiscordIpcClient::new(APPLICATION_ID);
                client = match candidate.connect() {
                    Ok(()) => Some(candidate),
                    Err(_) => None,
                };
                if client.is_none() { retry_at = Instant::now() + RETRY_INTERVAL; }
            }
            if let Some(connected) = client.as_mut() {
                let presence = current_presence();
                if last_presence.as_ref() != Some(&presence) || last_sent.elapsed() >= REFRESH_INTERVAL {
                    let activity = Activity::new()
                        .activity_type(ActivityType::Listening)
                        .details(&presence.details)
                        .state(&presence.state)
                        .buttons(vec![Button::new("Discover Saesth", WEBSITE_URL)]);
                    if connected.set_activity(activity).is_ok() {
                        last_presence = Some(presence);
                        last_sent = Instant::now();
                    } else {
                        let _ = connected.close();
                        client = None;
                        last_presence = None;
                        retry_at = Instant::now() + RETRY_INTERVAL;
                    }
                }
            }
        }
        thread::sleep(POLL_INTERVAL);
    }
}