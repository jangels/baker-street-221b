
use serde::{Deserialize, Serialize};

/// 证据：感知层 (Wiggins) 传来的原始数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub source: String,      // e.g., "10.0.1.5" or "pod-xyz"
    pub event_type: String,  // e.g., "TCP_RETRANSMIT"
    pub severity: f64,       // 0.0 - 1.0
    pub timestamp: i64,
}

/// 假设：推理引擎 (Holmes) 的中间产物
#[derive(Debug, Clone)]
pub struct Hypothesis {
    pub suspect: String,
    pub description: String,
    pub probability: f64, // Posterior
}

/// 判决：最终输出
#[derive(Debug, Clone)]
pub struct Verdict {
    pub suspect: String,
    pub root_cause: String,
    pub confidence: f64,
    pub action: String,
}