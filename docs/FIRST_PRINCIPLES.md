# The Observatory — First Principles

**Status:** Foundational design philosophy  
**Established:** 2026-11-11  
**Purpose:** A durable decision framework, not a feature wish list, implementation specification, or claim of scientific authority.

> **Build a place where ideas can be discovered freely, examined rigorously, and revised without losing the reasons why.**

This document exists to outlive individual conversations, technologies and contributors. When a proposed feature, architectural pattern, interface choice or AI capability conflicts with these principles, the burden is on the proposal to explain why the exception improves the product. These principles themselves may be amended, but changes must be explicit, justified and recorded in [DECISIONS.md](DECISIONS.md).

## The premises

1. **Humans do not begin with structured knowledge.** They encounter fragments: a question, an image, a note, a webpage, a contradictory observation. Rigid intake requirements destroy discoveries before investigation begins.
2. **Recorded information is not knowledge merely because it has been saved.** Sources can be wrong; quotations can be out of context; multiple reports can repeat one origin; polished explanations can conceal gaps.
3. **Reasoning is revisable.** New evidence and better methods can change what seems plausible. A tool that silently rewrites its history destroys the ability to understand how conclusions developed.
4. **Attention and trust are scarce.** More collected material, more AI output and more graph connections can *reduce* understanding if they are noisy, fabricated or impossible to inspect.
5. **The user should remain able to work without the vendor.** Their information, relationships and research history should survive an offline session, a failed service, a changed model, or eventually a replacement interface.

These premises imply the following product principles.

## P1 — Discovery must be easy; belief must be difficult

Capture a bookmark, note, photo, document, feed item or thought with minimal ceremony. Do not demand a hypothesis, taxonomy or folder structure before saving it.

But **saving something must never imply that it is verified**.

- A captured item is a resource.
- A written assertion is a claim.
- A source passage or observation can become evidence for a claim, with a traceable relationship.
- A conclusion is an assessment that remains open to reconsideration.

**Design test:** Can a user save a fleeting thought in seconds, while the application still prevents it from appearing as established evidence?

## P2 — Provenance is part of the data, not a footnote

An assertion is less useful when its origin cannot be reconstructed. Evidence records should identify the source, its version, precise locator, observed or publication timestamps as distinct facts, and the origin of the assertion.

When provenance cannot be established, state **unknown**. Never fabricate it through a migration, an AI response or a convenient default. A content hash can establish byte equality, not truth or authenticity.

**Design test:** Can the application show what material a claim relied on *at the time it was evaluated*, even after the original URL changes?

## P3 — Preserve disagreement and uncertainty

Contradictory evidence is information, not noise to average away. Support, contradiction, missing data, ambiguity and unresolved identity should be first-class states.

Do not turn uncalibrated intuition into a percentage. Do not count derivative articles as independent corroboration. Do not equate consensus, volume of references or institutional authority with proof.

**Design test:** Can two competing explanations and their strongest counterevidence be inspected side-by-side without the system forcing a premature winner?

## P4 — One knowledge model, many useful perspectives

People think differently at different stages. A quick-capture inbox, a document reader, an infinite canvas, a mind map, a table, a knowledge graph, a timeline and an evidence report should be **views over shared canonical records**.

A spatial association ("these ideas remind me of each other") is not the same as an evidentiary relationship ("this observation contradicts this claim"). Both may be useful; they must not be conflated.

**Design test:** Does moving a card between canvases preserve one underlying resource and its history?

## P5 — Corrections should improve history, not erase it

A source may change. An interpretation may improve. A claim may be withdrawn. Preserve the relevant history of what was asserted, by whom or by which process, and based on which source version—subject to legitimate privacy and deletion requirements.

Prefer explicit versions, append-only events for meaningful changes, and understandable tombstone/deletion policies. Don't use "immutable" as an excuse to retain information unlawfully or against a valid deletion request.

**Design test:** Can a user ask "what changed, when, and why?" and get an answer grounded in stored records rather than retrospective storytelling?

## P6 — AI assists investigation; it does not appoint itself the authority

AI is good at suggesting connections, extracting candidates and explaining complicated material. It can also hallucinate references, collapse nuance and sound certain without grounds.

- AI output is attributed to its model/run when relevant.
- Extracted claims and proposed connections remain proposals until reviewed.
- Every cited passage must be independently retrievable or flagged unverified.
- Core browsing, editing, linking, searching and export must work without an LLM.
- Models and providers can be replaced without losing user data.

**Design test:** If the AI is disabled or wrong, can the user still reconstruct, challenge and correct the investigation?

## P7 — A local-first system should not quietly become a surveillance system

The default is user-controlled local storage. Importing a resource, checking a feed, consulting an archive, or sending text to a remote model are distinct actions with clear network and privacy consequences.

Use least privilege, opt-in external requests, safe parsing and bounded automation. Support legitimate research and public-interest OSINT; reject indiscriminate collection or invasive profiling of private individuals.

**Design test:** Can a user tell where their information resides, what leaves the device, and which actions an assistant is authorized to perform?

## P8 — Reproducibility outranks impressive presentation

A beautiful interface is valuable only when it clarifies work. A connection line must express a meaningful relationship; an archive link must disclose capture limitations; a report must not disguise an unchecked assertion as confirmed evidence.

Favor inspection of source material, exact versions, explicit failures and exportable records over decorative graphs or summary scores.

**Design test:** Could another person follow the records and understand how a finding was reached, including its weak points?

## P9 — Complexity must be earned by a demonstrated need

Build the smallest vertical slice that tests a meaningful user outcome. Begin with one Rust domain core and SQLite, not seven distributed services or a generic plugin system.

The seven named capabilities—Archive, Atlas, Laboratory, Chronicle, Watchtower, Vault and Lens—are *responsibilities*, not instructions to implement seven services. The Codex is a presentation derived from the evidence graph, not a second authority.

For each new dependency or feature, answer:

1. Which existing user problem does it solve?
2. What smaller approach was considered?
3. Which new failure modes does it introduce?
4. What test or observation would prove it useful?
5. Can we remove it without corrupting core knowledge?

**Design test:** Would removing this abstraction make the system simpler without harming a real workflow? If so, remove it.

## P10 — Measure usefulness, not feature count

Success is not the number of AI agents, document types, connectors, visualizations or nodes in a graph. Success is whether a user can find information again, connect it correctly, inspect sources, identify uncertainty, revisit old conclusions and export work another person can follow.

Prefer small regression fixtures and reproducible end-to-end investigations. Keep honest status labels: **proposed**, **implemented**, **tested**, **verified**, and **released** are different states.

**Design test:** What observable task can now be completed reliably that could not be completed before?

## P11 — Learning must remain possible

The application should make it easy to change one's mind without punishment, shame or misleading certainty. Unresolved questions and failed hypotheses are not clutter; they are records of inquiry.

An encyclopedia-like Codex should surface evidence, alternatives and change history, not merely produce polished answers. A Watchtower should surface potentially relevant information, not silently rewrite settled claims.

**Design test:** Does the software make a correction easier to understand than simply overwriting the old answer?

## P12 — Interoperability is a means of user independence

Use documented local formats and export paths. Design one validated domain interface used by desktop and machine clients. A future MCP adapter should have scoped, auditable permissions and no unauthenticated public exposure.

An assistant should be able to *propose* a claim or request evidence without bypassing the exact checks applied to a human user. Remote connectivity is a separate explicit opt-in decision.

**Design test:** Could the user recover their research and continue working if the preferred desktop interface, AI provider or integration disappeared?

---

## Principles applied to concrete choices

| Proposed feature | Decision | Reason |
| --- | --- | --- |
| Fast bookmarking, notes and RSS | **Yes, phased** | Low-friction discovery; capture does not become proof |
| Infinite canvas and mind mapping | **Yes, phased** | Useful perspectives over canonical objects, not duplicate datasets |
| Wayback/Internet Archive links and local snapshots | **Yes, scoped** | Recoverability, version history, and source integrity |
| Public-interest OSINT tools | **Yes, scoped** | Source lineage, verification and entity analysis; no invasive profiling |
| Grokipedia-like readable articles | **Codex later** | Helpful derived view, provided claims and revisions remain inspectable |
| AI and machine access | **Optional, permissioned** | Assistive proposals; no silent unreviewed authority |
| General web crawler, social aggregation, shadow-library acquisition | **No for early versions** | Unfocused, rights/security/privacy risks, little need for first working slice |
| Graph database, microservices, cloud sync | **Defer** | No proven need over Rust + SQLite |
| Impressive dashboard without verifiable evidence | **Reject** | Performance theater is not investigation |

## How decisions are made

When a feature is proposed:

1. **State the user goal.** Do not begin with a technology or competitor's feature list.
2. **Find the smallest testable slice.** Define behavior, inputs, failure cases and observable output.
3. **Map its data and trust boundaries.** Distinguish source, assertion, interpretation and authority.
4. **Identify its dependencies and costs.** Include rights, offline operation, security and maintenance.
5. **Implement and test the core behavior first.** UI refinement follows usable functionality.
6. **Record the decision.** Use [DECISIONS.md](DECISIONS.md), a GitHub issue/PR and acceptance evidence.
7. **Revise if reality disagrees.** A foundational principle is a serious commitment, not an excuse to ignore better evidence.

### Priority when principles conflict

**Safety, lawful handling and integrity of user data** come first. Next are **truthful provenance and transparent uncertainty**. Then **user agency and offline durability**. Then **actual task usefulness**. Only then optimize convenience, visual polish and breadth. Document meaningful exceptions.

## Continuity contract

Before resuming development from a new conversation or contributor context:

1. Read **this file**, [VISION.md](VISION.md), [ARCHITECTURE.md](ARCHITECTURE.md), [ROADMAP.md](ROADMAP.md), and [DECISIONS.md](DECISIONS.md).
2. Inspect the actual GitHub issues, open pull requests, branch and CI results.
3. Distinguish committed plans from implemented and tested code.
4. Select the smallest open, dependency-ready acceptance gate.
5. Make a focused change, test it, report accurately, and update the docs when the reasoning changes.

**The Observatory should practice the epistemic discipline it exists to support.**
