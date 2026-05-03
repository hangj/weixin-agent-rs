use async_trait::async_trait;
use wechat_rs_sdk::{Agent, Bot, ChatRequest, ChatResponse, LoginOptions, MediaKind, MediaOutKind, MediaOutput, Result, StartOptions, auth::accounts::list_accounts};

struct EchoAgent;

#[async_trait]
impl Agent for EchoAgent {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        println!("on request: {:#?}", request);
        if let Some(media) = &request.media && media.kind == MediaKind::Image {
            let path = media.file_path.as_str();

            // todo: search this image
        }

        Ok(ChatResponse {
            text: Some(format!("你说了: {}", request.text)),
            media: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    if std::env::args().any(|a| a == "login") {
        let account_id = Bot::login(LoginOptions::default()).await?;
        println!("login success: {}", account_id);
        return Ok(());
    }

    let mut set = tokio::task::JoinSet::new();

    for account in list_accounts() {
        set.spawn(async move {
            println!("已登录账号 account_id: {}, user_id: {:?}", account.account_id, account.user_id);
            if let Err(err) = Bot::start(EchoAgent, StartOptions { account_id: Some(account.account_id.clone()) }).await {
                eprintln!("Bot(account_id: {}, user_id: {:?}) error: {err}", account.account_id, account.user_id);
            }
        });
    }

    while let Some(res) = set.join_next().await {
        if let Err(err) = res {
            eprintln!("Bot task error: {err}");
        }
    }

    // Bot::start(EchoAgent, StartOptions::default()).await?;

    Ok(())
}
