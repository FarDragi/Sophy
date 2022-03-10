use std::{collections::HashMap, sync::Arc, time::Duration};

use poise::{
    serenity_prelude::{Mutex as SerenityMutex, Ready, ShardManager},
    Framework, FrameworkOptions,
};
use tokio::{sync::Mutex, time::sleep};

use crate::{
    api::grpc::bootstrap_grpc_bot_client,
    commands::get_commands,
    config::Config,
    error::AppResult,
    events::event_listener,
    states::{grpc::GrpcServices, shard::ShardsLatency, States},
};

pub async fn bootstrap_bot(config: &Config) -> AppResult<()> {
    let options = FrameworkOptions {
        commands: get_commands(),
        listener: |ctx, event, framework, states| {
            Box::pin(event_listener(ctx, event, framework, states))
        },
        ..Default::default()
    };

    Framework::build()
        .options(options)
        .token(&config.token)
        .user_data_setup(move |_ctx, bot, framework| {
            Box::pin(async move {
                bot_info(bot);

                let shards_latency = Arc::new(Mutex::new(HashMap::new()));

                shard_latency(framework.shard_manager(), shards_latency.clone());

                Ok(States {
                    shards_latency,
                    grpc: GrpcServices {
                        bot: Mutex::new(bootstrap_grpc_bot_client().await?),
                    },
                })
            })
        })
        .run_autosharded()
        .await
        .expect("Fail start bot client");

    Ok(())
}

fn bot_info(bot: &Ready) {
    info!("Bot started: {}", bot.user.tag());
}

fn shard_latency(shards: Arc<SerenityMutex<ShardManager>>, latencies: ShardsLatency) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(60)).await;

            let shards_lock = shards.lock().await;
            let runners = shards_lock.runners.lock().await;

            for (id, runner) in runners.iter() {
                let latency = if let Some(latency) = runner.latency {
                    {
                        let mut latencies_lock = latencies.lock().await;
                        latencies_lock.insert(id.0, Arc::new(latency));
                    }

                    format!("a latency of {:?}", &latency)
                } else {
                    "no latency".to_string()
                };

                debug!("Shard [{}] is {} with {}", id, runner.stage, latency);
            }
        }
    });
}
