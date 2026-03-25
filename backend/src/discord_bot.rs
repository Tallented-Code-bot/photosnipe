use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::channel::{Message, Attachment};
use serenity::prelude::GatewayIntents;
use mongodb::Database;
use mongodb::bson::doc;

pub struct Bot {
    pub db: Database,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore bots
        if msg.author.bot {
            return;
        }

        // Get only image attachments
        let image_attachments: Vec<&Attachment> = msg.attachments 
            .iter()
            .filter(|a| a.width.is_some() && a.height.is_some())
            .collect();
        if image_attachments.is_empty() {
            return;
        }

        // If no one is mentioned, skip
        if msg.mentions.is_empty() {
            return;
        }

        let sniper_id = msg.author.id.0 as i64;
        let sniper_name = msg.author.name.clone();
        // available only if called with a context
        let sniper_nick = match msg.guild_id {
            Some(guild_id) => {
                if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
                    member.nick
                } else {
                    None
                }
            },
            None => None,
        };
        let channel_id = msg.channel_id.0 as i64;
        let guild_id = msg.guild_id.map(|g| g.0 as i64);
        let text = msg.content.clone();

        let persons_collection = self.db.collection::<mongodb::bson::Document>("persons");

        // Upsert sniper
        let sniper_filter = doc! { "id": sniper_id };
        let sniper_update = doc! {
            "$set": {
                "username": &sniper_name,
                "display_name": sniper_nick.as_ref(),
            }
        };
        let upsert_options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        let _ = persons_collection.update_one(sniper_filter, sniper_update, upsert_options).await;

        for snipee in msg.mentions {
            let snipee_id = snipee.id.0 as i64;
            let snipee_name = snipee.name.clone();
            // No nickname available directly; just pass None
            let snipee_nick: Option<String> = None;
            // Upsert snipee
            let snipee_filter = doc! { "id": snipee_id };
            let snipee_update = doc! {
                "$set": {
                    "username": &snipee_name,
                    "display_name": snipee_nick.as_ref(),
                }
            };
            let upsert_options = mongodb::options::UpdateOptions::builder()
                .upsert(true)
                .build();
            let _ = persons_collection.update_one(snipee_filter, snipee_update, upsert_options).await;
 
            // For each image, create a snipe
            let snipes_collection = self.db.collection::<mongodb::bson::Document>("snipes");
            for att in &image_attachments {
                let snipe_doc = doc! {
                    "sniper_id": sniper_id,
                    "snipee_id": snipee_id,
                    "picture_url": &att.url,
                    "text": &text,
                    "channel_id": channel_id,
                    "guild_id": guild_id,
                };
                let _ = snipes_collection.insert_one(snipe_doc, None).await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Discord bot connected as {}", ready.user.name);
    }
}

// Launch bot as independent async task
pub async fn start_discord_bot(db: Database, token: String) {
    println!("[discord_bot] Starting Discord bot background task...");
    let client_result = Client::builder(token.clone(), GatewayIntents::all())
        .event_handler(Bot { db })
        .await;
    match client_result {
        Ok(mut client) => {
            println!("[discord_bot] Serenity client created. Connecting to Discord...");
            if let Err(e) = client.start().await {
                eprintln!("[discord_bot][ERROR] Discord bot exited: {e}");
            }
        }
        Err(e) => {
            eprintln!("[discord_bot][ERROR] Could not create Discord client: {e}");
            eprintln!("[discord_bot][ERROR] Check your DISCORD_BOT_TOKEN and internet connectivity.");
        }
    }
}
