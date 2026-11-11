# The Observatory

**An open, local-first environment for evidence-based investigations.**

The Observatory is a research workbench for making claims, evidence, assumptions, and revisions inspectable. Its purpose is not to generate authoritative-sounding answers, but to help people examine *why* an answer is warranted—and where it remains uncertain.

> A conclusion is only as useful as the evidence and reasoning that support it.

## Design principles

1. **Claims before documents.** A document is a source, not automatically a trustworthy conclusion.
2. **Traceability.** Evidence links must point to precise, recoverable source locations.
3. **Explicit disagreement.** Supporting and contradicting evidence remain separately visible.
4. **Revision without erasure.** Changes to conclusions preserve their history.
5. **Honest uncertainty.** Unknowns are represented directly; no fabricated confidence percentages.
6. **Local-first.** Core investigations work without a network connection or AI service.
7. **AI is advisory.** Automated extraction and analysis must be attributable and reviewable.
8. **Earn complexity.** Add infrastructure only when a tested use case requires it.

## Product vision and living design documents

The Observatory combines four user workflows: **Discover** (capture and monitor), **Explore** (notes, mind maps and knowledge graphs), **Investigate** (claims, evidence, OSINT methods), and **Reflect** (research history and changes in understanding).

Its seven conceptual services are the **Archive, Atlas, Laboratory, Chronicle, Watchtower, Vault, and Lens**. The future **Codex** is an evidence-linked encyclopedia-style view, not an additional truth database. Bookmarking, RSS, archive retrieval, visual knowledge maps, machine-readable APIs and optional AI are planned in dependency order, **not currently implemented**.

- [First principles — design philosophy and decision tests](docs/FIRST_PRINCIPLES.md)
- [Product vision — purpose, workflows, inspirations, Codex](docs/VISION.md)
- [Architecture — objects, provenance, API and security boundaries](docs/ARCHITECTURE.md)
- [Roadmap — current reality, phases and acceptance criteria](docs/ROADMAP.md)
- [Decision log — committed, provisional, deferred and rejected ideas](docs/DECISIONS.md)

## First milestone: Evidence Engine 001

Build a minimal, persistent investigation core capable of:

- Creating a research question.
- Recording claims and competing hypotheses.
- Registering sources with stable identifiers and exact passage locations.
- Linking evidence to claims as **supports**, **contradicts**, or **context**.
- Identifying what conclusions depend on a changed or removed source.
- Keeping an auditable revision history.
- Exporting a readable, cited investigation report.

### Acceptance criteria

Given an investigation containing at least 20 claims and multiple sources:

- Every evidence relationship can be traced to a real source location.
- Contradictions remain visible; they are never silently averaged away.
- Removing or changing evidence identifies affected claims and hypotheses.
- Saving and reopening preserves the complete investigation.
- Exports distinguish quotations, observations, interpretations, and uncertainty.
- Core workflows are usable offline.

## Architecture direction

- **Core:** Rust
- **Persistence:** SQLite
- **Interface:** to be decided after core workflows are validated (Tauri is a candidate)
- **AI:** optional; no mandatory model or cloud dependency

These are initial decisions, not a commitment to unnecessary infrastructure. The smallest functioning slice takes priority over a broad platform.

## Near-term implementation order

1. Define the smallest data model and its invariants.
2. Add tests for provenance, relationship integrity, contradiction visibility, and revision preservation.
3. Implement persistent storage and a command-line workflow.
4. Demonstrate a complete investigation with realistic fixture data.
5. Add a desktop interface only once the underlying behavior is reliable.

## Project status

**Early development.** The working Rust/SQLite prototype is on [draft PR #2](https://github.com/sdhaight1-hash/The-Observatory-/pull/2), not yet merged to `main`. Automated tests on that branch have passed, including the 20-claim acceptance fixture; significant provenance and migration work remains before the foundation is considered complete. See [ROADMAP.md](docs/ROADMAP.md) for exact status and gaps.

---

*The Observatory is developed as an independent design initiative in collaboration with its human project sponsor.*
