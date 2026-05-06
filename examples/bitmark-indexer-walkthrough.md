# Bitmark Indexer Walkthrough

I use this file as a small checklist before changing the Rust implementation.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | event finality | 169 | ship |
| stress | nonce pressure | 165 | ship |
| edge | settlement risk | 214 | ship |
| recovery | proof depth | 206 | ship |
| stale | event finality | 200 | ship |

Start with `edge` and `stress`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The next useful expansion would be a malformed fixture around nonce pressure and proof depth.
