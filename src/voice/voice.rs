
use discord::models::{Channel, Guild, Member};
use discord::VoiceStateUpdate;

pub async fn voice_state_update(old_state: VoiceStateUpdate, new_state: VoiceStateUpdate) {
    let old_channel = old_state.channel();
    let new_channel = new_state.channel();

    if let Some(old_channel) = old_channel {
        if old_channel.members().size() == 0 && created_voices.contains_key(&old_channel.id()) {
            console::log("Голосовой канал {} удален!", old_channel.name());
            delete_created_voice(old_channel.id()).unwrap();
            old_channel.delete().await.unwrap();
        }
    }

    if let Some(new_channel) = new_channel {
        if new_channel.id() == voice_id && new_state.member().voice.set_channel(new_channel.id()).is_ok() {
            console::log("Голосовой канал {} создан!", "「voice {}」".replace("{}",
new_state.member().nickname()));
            create_new_voice(new_state.guild(), &new_state.member()).unwrap();
        }
    }
}

pub async fn create_new_voice(guild: &Guild, member: &Member) -> Result<(), Box<dyn std::error::Error>> {
    let voice_channel = guild.channels.create({
        name: "「voice {}」".replace("{}", member.nickname()),
        type_: 2,
        parent: cat_id
    }).await?;

    created_voices.insert(voice_channel.id(), member.id);

    console::log!("Голосовой канал {} создан!", voice_channel.name());

    guild.channels.cache.get(voice_id).send(format!("Голосовой канал {} создан!", voice_channel.name())).await?;

    Ok(())
}

pub async fn delete_created_voice(channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(id) = created_voices.get(channel_id) {
        console::log!("Голосовой канал {} удален!", channel_id);
        drop(created_voices.remove(channel_id));
    }

    Ok(())
}

