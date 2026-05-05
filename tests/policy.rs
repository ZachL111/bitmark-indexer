use bitmark_indexer::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 60, capacity: 73, latency: 11, risk: 6, weight: 10 };
    assert_eq!(score(signal), 184);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 83, capacity: 107, latency: 12, risk: 15, weight: 7 };
    assert_eq!(score(signal), 189);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 81, capacity: 99, latency: 10, risk: 6, weight: 13 };
    assert_eq!(score(signal), 273);
    assert_eq!(classify(signal), "accept");
}
