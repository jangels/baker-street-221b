use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};
use std::env;

/// 询问 Oracle (大模型)
/// 输入：症状 (Symptom), 嫌疑人 (Suspect), 关系 (Relation)
/// 输出：似然度 (Likelihood, 0.0 - 1.0)
pub async fn consult_oracle(symptom: &str, suspect: &str, victim: &str) -> f64 {
    // 1. 从环境变量获取 Key 和 URL
    let api_key = env::var("LLM_API_KEY").unwrap_or_default();
    let base_url = env::var("LLM_BASE_URL").unwrap_or("https://api.openai.com/v1".to_string());
    let model = env::var("LLM_MODEL").unwrap_or("gpt-3.5-turbo".to_string());

    if api_key.is_empty() {
        debug!("[LLM] No API Key found. Falling back to dummy logic.");
        return 0.5; // 没有 Key 就盲猜
    }

    // 2. 构造 Prompt (提示词工程)
    // 要求 AI 扮演一个 SRE 专家，评估故障概率
    let system_prompt = "You are a Site Reliability Engineer. \
        Analyze the likelihood that a failure in the 'Suspect' service caused the 'Symptom' in the 'Victim' service. \
        Return ONLY a JSON object: {\"likelihood\": 0.xx}. Do not explain.";

    let user_prompt = format!(
        "Victim Service: {}\nSuspect Service: {}\nSymptom: {}\n\
        Context: The Victim depends on the Suspect.\n\
        Task: Estimate P(Symptom | Suspect_Failure).",
        victim, suspect, symptom
    );

    // 3. 发送请求
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/chat/completions", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": 0.1 // 低温度，保证输出确定性
        }))
        .send()
        .await;

    // 4. 解析结果
    match resp {
        Ok(res) => {
            if let Ok(json_body) = res.json::<serde_json::Value>().await {
                // 提取 content 里的 JSON 字符串
                if let Some(content) = json_body["choices"][0]["message"]["content"].as_str() {
                    // 再次解析 content 里的 JSON (处理 AI 可能返回的 Markdown 格式)
                    let clean_json = content.trim().trim_matches('`').replace("json", "");
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&clean_json) {
                         if let Some(val) = parsed["likelihood"].as_f64() {
                             return val;
                         }
                    }
                }
            }
        }
        Err(e) => error!("[LLM] Request failed: {}", e),
    }

    error!("[LLM] Failed to parse likelihood. Defaulting to 0.1");
    0.1
}