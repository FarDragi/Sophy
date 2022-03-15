use std::collections::HashMap;

use poise::serenity_prelude::{Context, Message};

#[derive(Default)]
pub struct ExtraArgs {
    args: HashMap<String, String>,
}

impl ExtraArgs {
    pub fn add_arg(&mut self, key: String, value: String) {
        self.args.insert(key, value);
    }

    pub async fn format_args(&self, ctx: &Context, msg: &Message, text_raw: &str) -> String {
        let text = text_raw.chars().collect::<Vec<char>>();
        let mut result = "".to_string();
        let mut indexer = 0;

        'main_loop: loop {
            if text.len() == indexer {
                break;
            }

            if text[indexer] == '%' {
                let mut var_size = 0;
                let mut var_name = "".to_string();

                loop {
                    var_size += 1;

                    if text.len() == indexer + var_size {
                        result.push_str(&text_raw[indexer..text.len()]);
                        break 'main_loop;
                    }

                    if text[indexer + var_size] == '%' {
                        let var_result = self.switch_args(ctx, msg, &var_name).await;

                        if var_result != var_name {
                            result.push_str(&var_result);
                        } else {
                            result.push_str(&format!("%{}%", var_result));
                        }

                        indexer += var_size;
                        break;
                    } else {
                        var_name.push(text[indexer + var_size]);
                    }
                }
            } else {
                result.push(text[indexer])
            }

            indexer += 1;
        }

        result
    }

    async fn switch_args(&self, ctx: &Context, msg: &Message, arg: &str) -> String {
        match arg {
            "user" => msg.author.tag(),
            "username" => msg.author.name.to_string(),
            "usermention" => format!("<@{}>", msg.author.id),
            "id" => msg.author.id.to_string(),
            "avatar" => {
                if let Some(avatar) = msg.author.avatar_url() {
                    avatar
                } else {
                    msg.author.default_avatar_url()
                }
            }
            "membros" => {
                if let Some(guild) = msg.guild(ctx) {
                    guild.member_count.to_string()
                } else {
                    0.to_string()
                }
            }
            "idservidor" => {
                if let Some(guild) = msg.guild(ctx) {
                    guild.id.to_string()
                } else {
                    0.to_string()
                }
            }
            "server" => {
                if let Some(guild) = msg.guild(ctx) {
                    guild.name
                } else {
                    "".to_string()
                }
            }
            key => {
                if let Some(value) = self.args.get(key) {
                    value.to_owned()
                } else {
                    arg.to_owned()
                }
            }
        }
    }
}
