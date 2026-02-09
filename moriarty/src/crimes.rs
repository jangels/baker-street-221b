
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CrimeType {
    NetworkDelay { ms: u64 },
    PacketLoss { rate: f64 },
    KillProcess,
}