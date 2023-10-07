use std::sync::atomic::AtomicU8;
use std::time::Duration;

use discord_rich_presence::activity::Activity;
use discord_rich_presence::activity::Assets;
use discord_rich_presence::DiscordIpc;
use discord_rich_presence::DiscordIpcClient;

const DETAILS: [&str; 6] = [
    "Solving world hunger",
    "Creating a cure for cancer",
    "Lobbying for world peace",
    "Reversing Climate Change",
    "Eradicating Infectious Diseases",
    "Ensuring Universal Access to Education",
];

static INDEX: AtomicU8 = AtomicU8::new(0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1160301584635404299")?;

    client.connect()?;

    let activity_base = Activity::new().assets(
        Assets::new()
            .large_image("gigachad2")
            .large_text("A true gigachad"),
    );

    loop {
        let clone = activity_base.clone();
        let mut index = INDEX.load(std::sync::atomic::Ordering::Relaxed);

        if index > 5 {
            INDEX.store(0, std::sync::atomic::Ordering::Relaxed);
            index = 0;
        }

        println!("index: {}", index);
        println!("index as usize: {}", index as usize);

        if let Err(why) = client.set_activity(clone.details(DETAILS[index as usize])) {
            eprintln!("{}", why);
        };

        INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::sleep(Duration::from_millis(8000));
    }
}
