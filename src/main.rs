mod builtins;
mod user_defined;

use churro::prelude::*;
use dotenvy::dotenv;
use minijinja::Environment;
use minijinja::value::Serde;

use std::env;
use std::sync::Arc;

struct Hdlr {
    prefix: String,
    env: Arc<Environment<'static>>,
}

#[async_trait]
impl EventHandler for Hdlr {
    async fn message_sent(&self, ctx: Ctx) {
        let msg = {
            let Some(message) = &ctx.message else { return };
            message.body.as_deref().unwrap_or("").trim().to_owned()
        };

        let Some(msg) = msg.strip_prefix(&self.prefix) else {
            return;
        };

        let (cmd, args) = msg.split_once(' ').unwrap_or((msg, ""));
        builtins::cmd_entry(
            ctx,
            cmd.to_string(),
            args.to_string(),
            Arc::clone(&self.env),
        );
    }
}

#[tokio::main]
async fn main() -> CResult {
    dotenv().expect("couldn't load .env");
    let token = env::var("TOKEN").expect("TOKEN is not set");
    let server = env::var("SERVER").expect("SERVER is not set");
    let prefix = env::var("PREFIX").unwrap_or("$".to_string());
    let bot = Bot::login(&token, &server).await?;

    let mut env = user_defined::construct_env()
        .await
        .expect("Couldn't construct environment");

    let bot2 = Arc::new(bot.clone());

    let bot_user = Arc::clone(&bot2);
    env.add_function("get_user", move |user_id: String| {
        builtins::fetch_user(&bot_user.client, &user_id)
            .map(|user| Serde(user))
            .ok()
    });

    let bot_room = Arc::clone(&bot2);
    env.add_function("get_room", move |room_id: String| {
        builtins::fetch_room(&bot_room.client, &room_id)
            .map(|room| Serde(room))
            .ok()
    });

    bot.listen(Hdlr {
        prefix,
        env: Arc::new(env),
    })
    .await?;

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
