# The Observatory — Roadmap and Acceptance Gates

**Recorded:** 2026-11-11  
**Type:** Dependency-ordered plan, not schedule promises.

## Current verified state (at documentation update)

- **Main:** charter README, with no evidence engine merged yet.
- **Draft PR #2:** Rust + SQLite core, CLI, source-revision/citation pins, Markdown export and CI tests. Source: https://github.com/sdhaight1-hash/The-Observatory-/pull/2
- **CI:** passing for the latest head checked when this plan was written (`d285dfde`). This does **not** mean every product acceptance requirement is met.
- **Not implemented:** desktop app, real file ingestion, verified passage extraction, bookmarking, RSS, infinite canvas, OSINT integrations, Codex, AI adapters, MCP server.
- **Known open gaps:** user-supplied citation locators are not verified; deletions and full append-only auditing remain incomplete; schema-migration governance and source-content snapshots are not complete.
- Issue #1 remains the foundational milestone: https://github.com/sdhaight1-hash/The-Observatory-/issues/1

## Milestone 1 — Trusted evidence core (in progress)

**Build:** Rust + SQLite, typed investigation records, source-version-pinned evidence, basic offline CLI/report, migrations, independent test fixture.

**Exit gate:** (a) data integrity tests pass; (b) contradictory evidence survives; (c) source edits do not rewrite historical citations; (d) legacy records with lost provenance are visibly marked; (e) reopen/export a realistic 20+ claim investigation; (f) documented deletion/revision semantics; (g) tested database upgrades and backups; (h) important assertions don't rely on unverified passage positions without a visible warning.

**Next work:** review PR #2, implement a formal migration version strategy; source/passage verification and revision/deletion policy. Do not prematurely claim gate complete merely because CI is green.

## Milestone 2 — Useful desktop library

**Build:** Tauri + TypeScript/React shell, local resources, manual URL bookmark capture, simple notes, collections, browsing, basic FTS search, PDFs/text reading and exact citation capture.

**Exit gate:** users can import/save/search/reopen real material completely offline; reference an actual passage in a saved investigation; backup/restore and invalid inputs tested; no silent data corruption.

**Not yet:** full browser extension, AI agents, RSS, large graph engine.

## Milestone 3 — Visual exploration

**Build:** reusable research cards, infinite canvas, typed edges vs associative edges, backlinks, graph/list/table projections, investigation view.

**Exit gate:** dragging and linking objects across views never duplicates authoritative records; evidence type/locator inspection works in the canvas; reload preserves layout.

**Investigate UI objective:** users can inspect a claim with original passages, supporting/contradicting evidence and provenance.

**0.1 Research Workbench target:** milestones 1–3, as an actually usable local desktop research environment.

## Milestone 4 — Watchtower + Vault

**Build:** bookmark metadata refinements, RSS/Atom subscriptions, bounded refresh, dedupe; authorized research feeds; local source snapshots and hashing; Wayback/Internet Archive lookup, later optional Perma.cc; change comparisons.

**Exit gate:** snapshot a permitted page, link evidence to that exact version, detect a changed later version, preserve old citations, identify missing/incomplete historical versions. Feed network failures must not corrupt local state.

## Milestone 5 — Public-interest OSINT and investigation

**Build:** explicit entity resolution, typed and dated link analysis, source lineage, timelines, selected public-record APIs, optional image/metadata verification helpers.

**Exit gate:** a documented public-interest investigation can be reconstructed with recoverable sources, version-pinned citations, competing interpretations and uncertainty; no unauthorized mass profiling, scraping or surveillance.

## Milestone 6 — Optional AI assistance and the Living Model

**Build:** explicitly optional local/cloud adapters; source extraction proposals, suggested links, evidence-aware retrieval, synthesis and the Codex encyclopedia view. Eventually pre-registered predictions, subsequent observation comparisons and calibration.

**Exit gate:** generated content is clearly attributed and distinguishable from reviewed findings, citations resolve, user can reject proposed changes, offline/AI-disabled mode still works, and error rates are measured against a labeled corpus.

## Machine access (cross-cutting, not a separate product)

- Introduce stable internal typed service operations while developing milestones 1–2.
- First **MCP adapter** after core operations, permissions and auditing are stable, ideally during early desktop development.
- No default public network binding or unreviewed AI writes.
- External HTTP API only when justified by concrete integrations.

## Explicitly deferred

Autonomous web crawling; bulk shadow-library acquisition; social feed aggregation; cloud sync; real-time collaboration; mobile clients; multi-user permissions server; distributed processing; full Notion/Grokipedia/Maltego feature parity; generic office editing; predictive agents with untested authority.

## Delivery discipline

Each milestone should produce:
1. Small, reviewable pull requests.
2. Exact acceptance fixtures and regression tests.
3. User-visible demonstration or runnable workflow.
4. Documentation of what does **not** work yet.
5. A clear merge decision based on tested correctness, not feature count.
