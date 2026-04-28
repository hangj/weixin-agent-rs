use async_trait::async_trait;
use wechat_rs_sdk::{Agent, Bot, ChatRequest, ChatResponse, LoginOptions, MediaKind, MediaOutKind, MediaOutput, Result, StartOptions};

struct EchoAgent;

#[async_trait]
impl Agent for EchoAgent {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        println!("on request: {:#?}", request);
        let media = request.media.and_then(|m|{
            let kind = match m.kind {
                MediaKind::Image => MediaOutKind::Image,
                MediaKind::Audio => return None,
                MediaKind::Video => MediaOutKind::Video,
                MediaKind::File => MediaOutKind::File,
            };
            Some(MediaOutput { kind, url: m.file_path, file_name: None })
        });

        Ok(ChatResponse {
            text: Some(format!("你说了: {}", request.text)),
            media,
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

    Bot::start(EchoAgent, StartOptions::default()).await
}
