use std::{sync::atomic::AtomicU8, time::Duration};

use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

const DETAILS: [&str; 6] = [
    "Solving world hunger",
    "Creating a cure for cancer",
    "Lobbying for world peace",
    "Reversing Climate Change",
    "Eradicating Infectious Diseases",
    "Fighting Crime (I'm Batman)",
];

static INDEX: AtomicU8 = AtomicU8::new(0);

/// Function that will infinitely attempt to reconnect to Discord on connection failure.
fn attempt_reconnection(client: &mut DiscordIpcClient) {
    loop {
        info!("Connection lost, reconnecting...");
        if client.connect().is_ok() {
            info!("Successfully reconnected to Discord.");
            break;
        } else {
            error!("Reconnection attempt failed. Trying again in 3 seconds.");
        }
        std::thread::sleep(Duration::from_secs(3));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut client = DiscordIpcClient::new("1160301584635404299")?;

    let activity_base = Activity::new().assets(
        Assets::new()
            .large_image("gigachad2")
            .large_text("A true gigachad"),
    );

    if let Err(e) = client.connect() {
        error!("Error connecting to Discord: {}", e);
        attempt_reconnection(&mut client);
    };

    info!("Successfully connected to Discord.");

    loop {
        let clone = activity_base.clone();
        let mut index = INDEX.load(std::sync::atomic::Ordering::Relaxed);

        if index > 5 {
            INDEX.store(0, std::sync::atomic::Ordering::Relaxed);
            index = 0;
        }

        info!("Switching activity to: \"{}\".", DETAILS[index as usize]);

        if let Err(why) = client.set_activity(clone.details(DETAILS[index as usize])) {
            error!("ERROR: {}", why);
            attempt_reconnection(&mut client);
        };

        INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::sleep(Duration::from_millis(8000));
    }
}
