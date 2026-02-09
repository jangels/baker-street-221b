
pub mod graph;
use graph::{ServiceNode, DependencyEdge};
use common::Evidence;
use anyhow::Result;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use tracing::info;

pub struct MindPalace {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl MindPalace {
    pub async fn connect() -> Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        // 👇 这是一个特例，为了致敬原著，我们要打破工程惯例
        // 使用 PascalCase 或者保留大写后缀
        let ns = "Baker_Street"; // 既然要追求完美，这里也可以大写首字母
        let db_name = "Room_221B"; // 这里必须大写！

        db.use_ns(ns).use_db(db_name).await?;

        // 这里的 Log 会看起来非常赏心悦目
        info!("[Mycroft] Connected to MindPalace at '{} / {}'", ns, db_name);
        
        Ok(Self { db })
    }

    /// 初始化世界拓扑 (模拟从 K8s 同步)
    pub async fn sync_topology(&self) -> Result<()> {
        info!("[Mycroft] Syncing topology graph...");
        
        // 创建节点
        let payment: ServiceNode = self.db.create(("service", "payment")).content(ServiceNode {
            id: None, name: "payment".into(), business_value: 0.9, self_failure_prior: 0.05
        }).await?.unwrap();
        
        let redis: ServiceNode = self.db.create(("service", "redis")).content(ServiceNode {
            id: None, name: "redis".into(), business_value: 0.6, self_failure_prior: 0.01
        }).await?.unwrap();

        // 创建边：Payment 依赖 Redis
        let sql = "RELATE service:payment->depends_on->service:redis SET p_propagation = 0.95";
        self.db.query(sql).await?;
        
        Ok(())
    }

    /// 提取上下文：根据证据找嫌疑人
    pub async fn get_suspects(&self, evidence: &Evidence) -> Result<Vec<(String, f64)>> {
        // 简单逻辑：如果 evidence source 是 payment，查找它依赖谁
        let victim = &evidence.source;
        
        // SurrealQL: 查找所有 payment 依赖的服务 (outbound)
        let sql = "SELECT out.name as name, p_propagation FROM depends_on WHERE in.name = $victim";
        let mut resp = self.db.query(sql).bind(("victim", victim)).await?;
        
        // 解析结果 (Mock)
        #[derive(serde::Deserialize)]
        struct Row { name: String, p_propagation: f64 }
        let rows: Vec<Row> = resp.take(0)?;
        
        // 返回 (Suspect Name, Prior Probability based on propagation)
        Ok(rows.into_iter().map(|r| (r.name, r.p_propagation)).collect())
    }
}