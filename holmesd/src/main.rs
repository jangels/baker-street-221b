pub mod llm;
mod engine;
use engine::DeductionEngine;
use mycroft::MindPalace;
use wiggins::Irregulars;
use watson::Doctor;
use anyhow::Result;
use tracing::info;
use dotenv::dotenv; 

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // 👈 加载 .env 文件

    tracing_subscriber::fmt::init();
    info!("🎻 The game is afoot! holmesd starting...");

    // 1. 连接大脑
    let mind = MindPalace::connect().await?;
    mind.sync_topology().await?;

    // 2. 部署探针
    let mut sensor_stream = Irregulars::deploy().await;
    
    // 3. 准备引擎与执行器
    let engine = DeductionEngine::new(&mind);
    let watson = Doctor::new();

    // 4. 事件循环
    while let Some(evidence) = sensor_stream.recv().await {
        info!("[Holmes] Analyzing evidence: {:?}", evidence);
        
        if let Some(verdict) = engine.abduce(&evidence).await? {
            info!("[Holmes] Deduction complete. Handing over to Watson.");
            watson.execute(&verdict).await;
        } else {
            info!("[Holmes] Data insufficient. Continuing surveillance.");
        }
    }

    // 👇 加上这一行：给数据库一点时间优雅退出
    // 就像关电脑前等待 Windows 更新一样
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("👋 Holmesd shutting down gracefully.");

    Ok(())
}