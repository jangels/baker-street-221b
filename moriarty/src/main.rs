
mod crimes;
use crimes::CrimeType;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("👹 Moriarty Chaos Engine Initialized.");
    println!("    Targeting: Production Cluster");

    loop {
        println!("[Moriarty] Plotting scheme...");
        sleep(Duration::from_secs(5)).await;
        
        let crime = CrimeType::NetworkDelay { ms: 500 };
        println!("[Moriarty] Executing Crime: {:?} on service 'redis'", crime);
        
        // 实际上这里会调用 chaos-mesh API 或 tc 命令
    }
}