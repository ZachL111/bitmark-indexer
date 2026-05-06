#[derive(Clone, Copy, Debug)]
pub struct Signal {
    pub demand: i32,
    pub capacity: i32,
    pub latency: i32,
    pub risk: i32,
    pub weight: i32,
}

pub const THRESHOLD: i32 = 158;
const RISK_PENALTY: i32 = 6;
const LATENCY_PENALTY: i32 = 3;
const WEIGHT_BONUS: i32 = 6;

pub fn score(signal: Signal) -> i32 {
    signal.demand * 2 + signal.capacity + signal.weight * WEIGHT_BONUS
        - signal.latency * LATENCY_PENALTY
        - signal.risk * RISK_PENALTY
}

pub fn classify(signal: Signal) -> &'static str {
    if score(signal) >= THRESHOLD { "accept" } else { "review" }
}

pub mod domain_review;
