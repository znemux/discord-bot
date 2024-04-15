use poise::serenity_prelude as serenity;
mod bot; use bot::*;

#[tokio::main]
async fn main() {
    let token = get_token();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

fn get_token() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 { return args[1].parse().unwrap(); }
    return std::env::var("DISCORD_TOKEN")
        .expect("Missing argument or environment variable DISCORD_TOKEN");
}