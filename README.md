# bitmark-indexer

`bitmark-indexer` is a compact Rust repository for blockchain tooling, centered on this goal: Index block headers, chain scores, and reorg decisions.

## Why I Keep It Small

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how event finality and settlement risk should influence a review result.

## Bitmark Indexer Review Notes

`edge` and `stress` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Included Behavior

- `fixtures/domain_review.csv` adds cases for event finality and nonce pressure.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/bitmark-indexer-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `settlement risk` and `nonce pressure`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Internal Model

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The Rust addition stays small enough to inspect in one sitting.

## Try It Locally

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Validation

The check exercises the source code and the review fixture. `edge` is the high score at 214; `stress` is the low score at 165.

## Scope

The repository is intentionally scoped to local checks. I would expand it by adding adversarial fixtures before adding features.
