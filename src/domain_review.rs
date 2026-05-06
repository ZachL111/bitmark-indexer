#[derive(Clone, Copy, Debug)]
pub struct DomainCase {
    pub signal: i32,
    pub slack: i32,
    pub drag: i32,
    pub confidence: i32,
}

pub fn review_score(case: DomainCase) -> i32 {
    case.signal * 2 + case.slack + case.confidence - case.drag * 3
}

pub fn review_lane(case: DomainCase) -> &'static str {
    let score = review_score(case);
    if score >= 140 { "ship" } else if score >= 105 { "watch" } else { "hold" }
}
