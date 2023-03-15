use log::info;
use serenity::async_trait;

use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::prelude::interaction::Interaction::ApplicationCommand;
use serenity::model::interactions::*;
use shuttle_service::error::CustomError;
use shuttle_service::SecretStore;
use sqlx::PgPool;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(＜DISCORD_GUILD_ID＞);

　　　　　　　　　　　　　　　　//コマンドを登録
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("hello").description("Say hello")
            }).create_application_command(|command| {
                command.name("bye").description("Say Good Bye")
            })
        })
        .await
        .unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self,ctx: Context,interaction: serenity::model::interactions::Interaction) {
        if let ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello!!".to_owned(),
                "bye" => "good bye!!".to_owned(),
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}