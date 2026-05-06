use bitmark_indexer::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 72, slack: 41, drag: 26, confidence: 62 };
    assert_eq!(review_score(case), 169);
    assert_eq!(review_lane(case), "ship");
}
