use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity_framework::prelude::*;

use std::env;

#[derive(Debug)]
pub struct TestData {
    text: String,
}

#[check]
async fn role(ctx: &CheckContext<'_, TestData>, msg: &Message) -> CheckResult {
    let member = msg.member(ctx).await.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    let test = guild.role_by_name("test").unwrap();
    if member.roles.contains(&test.id) {
        Ok(())
    } else {
        Err(Reason::User("you do not possess the test role".to_string()))
    }
}

/// AAAAAAAA
#[command]
#[subcommands(pong)]
#[delimiter(",")]
async fn ping(
    ctx: FrameworkContext<TestData>,
    msg: &Message,
    m: i32,
    n: Option<String>,
    o: Vec<u64>,
) -> CommandResult {
    let text = &ctx.data.text;
    let message = format!(
        "Text: \"{}\"\nCommand ID: {:?}\nPrefix: \"{}\"\nArgs: \"{}\"\nm: {}\nn: {:?}\no: {:?}",
        text, ctx.command_id, ctx.prefix, ctx.args, m, n, o
    );
    msg.channel_id.say(&ctx, message).await?;
    Ok(())
}

/// BBBBBBBBBBBBB
#[command("pong", "pang")]
#[description("hello")]
#[check(role)]
pub async fn pong(ctx: FrameworkContext<TestData>, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx,
            format!(
                "State of the framework's configuration: ```rust\n{:?}\n```",
                ctx.conf
            ),
        )
        .await?;
    Ok(())
}

#[command]
#[subcommands(boop)]
async fn f(ctx: FrameworkContext<TestData>, msg: &Message) -> CommandResult<()> {
    _ping(ctx, msg).await
}

#[command]
#[subcommands(ping)]
#[check(role)]
async fn boop(ctx: FrameworkContext<TestData>, msg: &Message) -> CommandResult<()> {
    _ping(ctx, msg).await
}

struct Handler {
    framework: Framework<TestData>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if let Some(guild_id) = msg.guild_id {
            if guild_id != 381880193251409931 && guild_id != 137234234728251392 {
                return;
            }
        }

        if let Err(err) = self.framework.dispatch(&ctx, &msg).await {
            match err {
                FrameworkError::Dispatch(DispatchError::CheckFailed(command, reason)) => {
                    match reason {
                        Reason::User(message) => {
                            let _ = msg
                                .channel_id
                                .say(
                                    &ctx,
                                    format!("command {} failed because \"{}\"", command, message),
                                )
                                .await;
                        }
                        _ => {}
                    }
                }
                err => {
                    eprintln!("error: {}", err);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut conf = Configuration::new();
    conf.prefix("???");
    conf.prefix("!!!");
    conf.dynamic_prefix(|_ctx, msg| {
        Box::pin(async move {
            if msg.content.starts_with("1234 ") {
                Some("1234 ".len())
            } else {
                None
            }
        })
    });
    conf.case_insensitive(true);
    conf.on_mention(136510335967297536);
    conf.category("general", &[ping]);
    conf.category("Sprint", &[f]);
    conf.command(pong);

    let framework = Framework::with_data(
        conf,
        TestData {
            text: "42 is the answer to life, the universe, and everything".to_string(),
        },
    );

    let mut client = Client::builder(&token)
        .event_handler(Handler { framework })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
