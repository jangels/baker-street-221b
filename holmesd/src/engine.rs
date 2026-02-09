
use common::{Evidence, Hypothesis, Verdict};
use mycroft::MindPalace;
use anyhow::Result;
use tracing::info;
use crate::llm;

pub struct DeductionEngine<'a> {
    mind: &'a MindPalace,
}

/// When you have eliminated the impossible, whatever remains, however improbable, must be the truth
impl<'a> DeductionEngine<'a> {
    pub fn new(mind: &'a MindPalace) -> Self {
        Self { mind }
    }

    /// 溯因推理循环 (The Abductive Loop)
    pub async fn abduce(&self, evidence: &Evidence) -> Result<Option<Verdict>> {
        // 1. 获取先验 (Yesterday's Posterior)
        let suspects = self.mind.get_suspects(evidence).await?;
        
        if suspects.is_empty() {
            return Ok(None);
        }

        let mut hypotheses = Vec::new();

        // 2. 贝叶斯计算
        for (name, prior_p) in suspects {
            // P(E|H): 似然度 (Likelihood)
            // 这里通常调用 LLM，现在用模拟逻辑
            // 假设：如果 Redis 挂了，导致 TCP 重传的概率极高 (0.95)
            let likelihood = if name == "redis" { 0.95 } else { 0.1 };
            
            /*
            // TODO 
            // 🛑 删除旧代码: let likelihood = if name == "redis" { 0.95 } else { 0.1 };
            // ✅ 新代码: 真正的 AI 推理
            info!("[Engine] Consulting LLM about suspect: {} ...", name);
            
            let likelihood = llm::consult_oracle(
                &evidence.event_type, // e.g. "TCP_RETRANSMIT"
                &name,                // e.g. "redis"
                &evidence.source      // e.g. "payment"
            ).await;
            */
            
            // P(H|E) = P(H) * P(E|H) / P(E) -> 简化为非归一化分数
            let posterior_score = prior_p * likelihood;
            
            info!("[Engine] Suspect: {}, Prior: {}, Likelihood: {}, Posterior: {}", 
                 name, prior_p, likelihood, posterior_score);

            hypotheses.push(Hypothesis {
                suspect: name,
                description: "Dependency failure".into(),
                probability: posterior_score,
            });
        }

        // 3. 排序与判决
        hypotheses.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
        
        if let Some(best) = hypotheses.first() {
            if best.probability > 0.8 {
                return Ok(Some(Verdict {
                    suspect: best.suspect.clone(),
                    root_cause: "Cascading Failure via Dependency".into(),
                    confidence: best.probability,
                    action: format!("Rolling Restart deployment/{}", best.suspect),
                }));
            }
        }

        Ok(None)
    }
}