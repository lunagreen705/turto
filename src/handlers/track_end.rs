use serenity::{async_trait, model::prelude::GuildId, prelude::Context};
use songbird::events::{Event, EventContext, EventHandler};
use tracing::error;

use crate::{
    typemap::config::GuildConfigs,
    utils::play::{play_next, PlayError},
};

pub struct TrackEndHandler {
    pub ctx: Context,
    pub guild_id: GuildId,
}

#[async_trait]
impl EventHandler for TrackEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if let Err(PlayError::EmptyPlaylist(_guild)) = play_next(&self.ctx, self.guild_id).await {
            let guild_configs_lock = self
                .ctx
                .data
                .read()
                .await
                .get::<GuildConfigs>()
                .unwrap()
                .clone();
            let auto_leave = {
                let mut guild_configs = guild_configs_lock.lock().await;
                let guild_config = guild_configs.entry(self.guild_id).or_default();
                guild_config.auto_leave
            };
            if auto_leave {
                let manager = songbird::get(&self.ctx).await.unwrap().clone();

                if let Err(e) = manager.remove(self.guild_id).await {
                    error!("Error leave voice channel: {}", e);
                }
            }
        }
        None
    }
}
