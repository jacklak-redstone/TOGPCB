mod builtins;
mod user_defined;

use churro::prelude::*;
use dotenvy::dotenv;

use std::env;

struct Hdlr {
    prefix: String,
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
        builtins::cmd_entry(ctx, cmd.to_string(), args.to_string());
    }
}

#[tokio::main]
async fn main() -> CResult {
    dotenv().expect("couldn't load .env");
    let token = env::var("TOKEN").expect("TOKEN is not set");
    let server = env::var("SERVER").expect("SERVER is not set");
    let prefix = env::var("PREFIX").unwrap_or("$".to_string());
    let bot = Bot::login(&token, &server).await?;

    user_defined::construct_env()
        .await
        .expect("Couldn't construct environment");

    bot.listen(Hdlr { prefix }).await?;

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
