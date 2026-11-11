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

**Foundation / specification phase.** No functioning evidence engine is claimed yet.

---

*The Observatory is developed as an independent design initiative in collaboration with its human project sponsor.*
