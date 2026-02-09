
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNode {
    pub id: Option<Thing>,
    pub name: String,
    pub business_value: f64, // 影子价格
    pub self_failure_prior: f64, // 自身故障先验概率 P(H)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyEdge {
    #[serde(rename = "in")]
    pub source: Thing,
    #[serde(rename = "out")]
    pub target: Thing,
    pub p_propagation: f64, // P(E|H) 传播概率
}