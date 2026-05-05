use bitmark_indexer::{classify, score, Signal};

fn main() {
    let signal = Signal { demand: 80, capacity: 95, latency: 12, risk: 8, weight: 7 };
    println!("score={} decision={}", score(signal), classify(signal));
}
