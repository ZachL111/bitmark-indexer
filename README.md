# bitmark-indexer

bitmark-indexer is a Rust project for blockchain tooling. It focuses on this technical goal: Index block headers, chain scores, and reorg decisions.

## Why it exists

Small engineering tools are easiest to trust when their rules are explicit, testable, and cheap to run locally. This repository packages a focused model with fixture data and a local verification path so behavior can be reviewed without external services.

## Features

- Deterministic policy scoring over fixture scenarios.
- Clear accept or review decisions based on a documented threshold.
- A command-line or local test path for quick validation.
- Golden fixture data for repeatable checks.
- Minimal dependencies and a compact project layout.

## Architecture Notes

The core module exposes a small scoring API. Inputs are simple numeric signals: demand, capacity, latency, risk, and weight. The score uses a threshold of 158, risk penalty 6, latency penalty 3, and weight bonus 6. Tests exercise the public API against the fixture cases in `fixtures/cases.csv`.

## Setup

Install the Rust toolchain and run commands from the repository root.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

The verification script builds or runs the project and checks the fixture decisions.

## Tests

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Limitations And Roadmap

- The fixture set is intentionally small so it can be audited by hand.
- Future work could add richer domain-specific input adapters.
- The model is a local demonstration and does not claim production use.
