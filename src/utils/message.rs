use poise::serenity_prelude::Message;

pub trait IsBotMessage {
    fn is_bot_message(&self) -> bool;
}

impl IsBotMessage for Message {
    fn is_bot_message(&self) -> bool {
        self.author.bot
    }
}
