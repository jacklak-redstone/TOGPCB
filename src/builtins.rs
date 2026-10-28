use churro::prelude::*;
use churro::proto::chatto::api::v1::*;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub(crate) fn cmd_entry(ctx: Ctx, cmd: String, args: String) {
    rayon::spawn(move || {
        if let Some(btin) = builtin_cmd().get(&cmd) {
            let Some(message) = &ctx.message else { return };

            let _ = reply_in_thread(&ctx.bot.client, message, &btin(args), false);
        } else {
        }
    });
}

fn builtin_cmd() -> HashMap<String, fn(String) -> String> {
    static MAP: OnceLock<HashMap<String, fn(String) -> String>> = OnceLock::new();

    MAP.get_or_init(|| {
        let mut hm = HashMap::new();
        hm.insert(
            "version".to_string(),
            version as fn(std::string::String) -> std::string::String,
        );
        hm.insert(
            "help".to_string(),
            help as fn(std::string::String) -> std::string::String,
        );
        hm
    })
    .clone()
}

fn version(_: String) -> String {
    "TOGPCB 0.1.0".to_string()
}

fn help(topic: String) -> String {
    match topic
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>()
        .as_str()
    {
        "" => "Hi there! Need help with TOGPCB? Try running me again with a specific topic. :)",
        "src" | "source" => "https://github.com/jacklak-redstone/TOGPCB (note: not public yet :()",
        _ => "Sorry, I can't help with that. :(",
    }
    .to_string()
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime")
    })
}

// DO NOT CALL THE FOLLOWING WRAPPERS FROM ASYNC CODE!!!
fn fetch_room(client: &Client, room_id: &str) -> CResult<Room> {
    runtime().block_on(Client::fetch_room(client, room_id))
}

fn fetch_user(client: &Client, user_id: &str) -> CResult<User> {
    runtime().block_on(Client::fetch_user(client, user_id))
}

fn fetch_msg(client: &Client, room_id: &str, message_id: &str) -> CResult<Message> {
    runtime().block_on(Client::fetch_msg(client, room_id, message_id))
}

fn send_msg(client: &Client, room_id: &str, msg: &str) -> CResult {
    runtime().block_on(Client::send_message_raw(client, room_id, msg))
}

fn reply_in_thread(
    client: &Client,
    message: &Message,
    body: &str,
    send_to_channel: bool,
) -> CResult {
    runtime().block_on(Client::reply_in_thread(
        client,
        message,
        body,
        send_to_channel,
    ))
}
