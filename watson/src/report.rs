use common::Verdict;

pub fn generate_markdown(verdict: &Verdict) -> String {
    format!(
        "# 🕵️ Baker Street Incident Report\n\n**Suspect**: `{}`\n**Confidence**: `{:.2}%`\n**Cause**: {}\n\n> Action Required: {}",
        verdict.suspect,
        verdict.confidence * 100.0,
        verdict.root_cause,
        verdict.action
    )
}