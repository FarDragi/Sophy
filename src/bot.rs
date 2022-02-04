use std::{sync::Arc, time::Duration};

use poise::{
    serenity_prelude::{Mutex, Ready, ShardManager},
    Framework, FrameworkOptions,
};
use tokio::time::sleep;

use crate::{commands::get_commands, config::Config};

pub async fn bootstrap_bot(config: &Config) {
    let options = FrameworkOptions {
        commands: get_commands(),
        ..Default::default()
    };

    Framework::build()
        .options(options)
        .token(&config.token)
        .user_data_setup(move |_ctx, bot, framework| {
            Box::pin(async move {
                bot_info(bot);
                shard_latency(framework.shard_manager());

                Ok(())
            })
        })
        .run_autosharded()
        .await
        .expect("Fail start bot client");
}

fn bot_info(bot: &Ready) {
    println!("Bot started: {}", bot.user.tag());
}

fn shard_latency(shards: Arc<Mutex<ShardManager>>) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;

            let shards_lock = shards.lock().await;
            let runners = shards_lock.runners.lock().await;

            for (id, runner) in runners.iter() {
                let latency = if let Some(latency) = runner.latency {
                    format!("a latency of {:?}", latency)
                } else {
                    "no latency".to_string()
                };

                println!("Shard [{}] is {} with {}", id, runner.stage, latency);
            }
        }
    });
}
