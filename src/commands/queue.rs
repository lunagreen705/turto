use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::guild::playlist::{Playlist, Playlists, Metadata};

#[command]
async fn queue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let url = args.rest();
    let source = songbird::input::ytdl(&url).await?;
    let metadata = source.metadata.clone();

    let playlists_lock = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Playlists>()
            .expect("Expected Playlists in TypeMap.")
            .clone()
    };
    {
        let mut playlists = playlists_lock.lock().await;
        let playlist = playlists
            .entry(msg.guild_id.expect("Expected guild_id"))
            .or_insert_with(Playlist::new);

        msg.reply(ctx, format!("✅ {}", metadata.title.clone().unwrap()))
            .await?;
        playlist.push_back(Metadata::from(*metadata)); // Add song to playlist
    }
    Ok(())
}
