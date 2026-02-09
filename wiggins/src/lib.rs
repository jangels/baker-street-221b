use common::Evidence;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};
use tracing::info;
use rand::Rng; // 引入随机数生成器

pub struct Irregulars;

impl Irregulars {
    pub async fn deploy() -> Receiver<Evidence> {
        let (tx, rx) = mpsc::channel(100);
        info!("[Wiggins] eBPF Probes loaded. Starting continuous surveillance...");

        tokio::spawn(async move {
            // 👇 加上这个 loop，让它永远不退出
            loop {
                // 模拟：随机等待 5~10 秒，假装这是真实环境中的偶发故障
                let delay = rand::thread_rng().gen_range(5..10);
                sleep(Duration::from_secs(delay)).await;
                
                let ev = Evidence {
                    source: "payment".to_string(),
                    event_type: "TCP_RETRANSMIT_SPIKE".to_string(),
                    severity: 0.8,
                    timestamp: chrono::Utc::now().timestamp(),
                };
                
                info!("[Wiggins] 🚨 Intercepted raw signal from Kernel: {:?}", ev);
                
                // 如果发送失败（比如主程序挂了），则退出循环
                if tx.send(ev).await.is_err() {
                    break;
                }
            }
        });

        rx
    }
}