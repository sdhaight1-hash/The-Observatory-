# The Observatory — Architecture and Invariants

**Status:** Architectural direction; evolving. Only the code in the repository and passing tests establish implemented behavior.

## Architectural rule

**One core, multiple projections.** The same resource/claim/entity may appear in an inbox, document viewer, whiteboard, graph, database view, timeline, evidence report, or Codex article. Presentation placement must not create conflicting copies of authoritative data.

The backend owns identity, validation, persistence, migrations, relationship rules, provenance and revision history. UI, automation and AI adapters call the same domain operations.

## Proposed stack

- **Rust core** — shared domain library and application services (already prototyped in draft PR #2).
- **SQLite** — local storage, transactions, referential integrity and eventually FTS5.
- **Local file store** — user-managed imports/snapshots with content hashing and clear content rights.
- **Desktop app:** Tauri preferred, React + TypeScript preferred for user interface; not yet implemented.
- **Canvas:** evaluate React Flow / XYFlow for node/canvas interactions; evaluate a dedicated renderer if graph scale demands it.
- **Automation:** scheduled, bounded RSS and source checks only after a reliable capture pipeline.
- **AI integration:** optional adapters, with separate *proposal* and *approval* operations; deterministic core must work without AI.
- **Machine access:** a documented Rust service interface first; then a local/authorized MCP adapter. HTTP API only if real integrations require it.

No mandatory distributed system, server, vector database, graph database, or microservices. Reconsider only after measured needs.

## Canonical domain concepts

| Type | Identity / semantics |
| --- | --- |
| Investigation | Research question with linked claims, sources, history and assessments |
| Resource | Stable identity for saved note, file, URL, image, dataset or bookmark |
| Source | Citable material, with provenance, author/publication metadata when known |
| SourceVersion | Immutable snapshot metadata: version, observed capture time, locator, optional content hash, retention rights |
| Passage | Exact source-version locator and optionally quoted text/content hash; verifies or explicitly states verification unavailable |
| Claim | Separately identified statement, including a kind (claim/hypothesis), scope and origin |
| EvidenceLink | Typed supports/contradicts/context relation from an explicit passage/observation to a claim |
| Entity | Public or conceptual subject and its identifiers; inferred identity matches are proposals until verified |
| Relationship | Typed, directional relation with provenance, validity window and review status |
| Collection | Grouping/organization without duplicating resources |
| View | Saved layout/configuration/positions referencing canonical IDs |
| Event/Revision | Append-only record of significant change, actor/origin, time and relevant before/after or version references |
| FeedSubscription / FeedItem | A selected RSS/Atom source and observed items, linked to resources through import/review |
| DerivedArticle | Codex presentation referencing claims and cited data; not an independent truth authority |

Start with the minimal investigation/source/claim/evidence model and expand with tested migrations; do not implement all types preemptively.

## Semantic boundaries

1. **Saved association is not evidence.** A freeform canvas line or suggested similarity is an *associative link*; only an explicitly typed, supported relation should enter the evidence graph.
2. **Source != claim != evidence.** A URL by itself does not validate a claim. Evidence requires a passage or observation, a locator, source identity/version and review status.
3. **Statement provenance matters.** User statements, third-party quotations, automated extraction, AI proposals and independently reviewed observations must be distinguishable.
4. **Citations are version-pinned.** Revising a source cannot retroactively point an older evidence link at newer content. If a legacy version cannot be reconstructed, mark it unresolved — never invent provenance.
5. **No fake confidence.** Do not assign arbitrary numbers without an explicit, calibrated method.
6. **No silent deletion of history.** Preserve old assessments, relationships and relevant versions; use explicit tombstones/retention decisions with sensitivity to data deletion rights.
7. **Deduplicate carefully.** Several articles quoting the same study do not constitute several independent confirmations.
8. **Separate times.** Publication date, event date, observed capture timestamp, archive capture date, ingestion time and revision time are distinct attributes.
9. **Corroboration is evaluated, not counted.** Contradictory records remain visible.

## User interfaces and access

**Human:** The desktop UI provides Discover / Explore / Investigate / Reflect.

**Machine:** tool endpoints (eventually MCP) expose typed methods such as `search_knowledge`, `get_investigation`, `add_source`, `propose_claim`, `link_evidence`, `get_dependencies`, `compare_versions` and `create_investigation`. These are **proposed names**, not currently implemented MCP endpoints.

**Security:** no unauthenticated public listener; read-only by default; explicit user consent for writes; least-privilege scopes, local transport where possible, audited mutations, safe import parsing, rate limits for network adapters, SSRF protections for URL fetching, and untrusted/archived HTML sandboxing. Never allow archive content, feed items or imported documents to issue instructions to the application or AI.

**Remote ChatGPT connectivity caveat:** a local-only Windows desktop process is not automatically reachable from hosted ChatGPT. A secure, user-authorized bridge or supported connector is required. Do not expose SQLite or an unauthenticated MCP endpoint directly to the public internet.

## The Watchtower

- Manual bookmark/URL capture before automatic subscriptions.
- RSS/Atom fetch with HTTP caching (ETag/Last-Modified), bounded polling, deduplication, source timestamps and failures surfaced.
- Optional research APIs (OpenAlex, Crossref, authorized repositories and public records); each connector has explicit constraints.
- Suggested relevance is a **proposal** and does not automatically alter accepted claims.

## The Vault

- Keep original URL, observed metadata, archival references and source version identity separately.
- Where lawful and available, retain local content and a SHA-256 hash of the *actual bytes captured*.
- Distinguish *URL changed*, *content changed*, *archive copy differs*, and *capture incomplete*.
- Resolve archived sources through Wayback Machine, Internet Archive and possibly Perma.cc using their supported mechanisms.
- Do not equate a hash, archived record, or timestamp with truth or authenticity.
- Gracefully represent missing, partially captured, paywalled or legally inaccessible documents.

## The Codex

A derived article should store links to claims and evidence spans with a known generation/version context. A sourced statement and an AI-generated summary should be visually distinguishable. Changes to linked claims should mark an article as potentially stale, not silently regenerate and replace it.

## Initial invariants and testing

- Foreign keys and cross-investigation constraints reject invalid evidence relationships.
- Citation version remains stable when source metadata changes.
- Support and contradiction coexist.
- Persistence and schema migrations preserve records, with unverifiable history explicitly flagged.
- A 20+ claim fixture with multiple hypotheses/sources can be stored, reloaded and exported offline.
- Additional gates before 0.1: real passage verification, deletion/tombstone semantics, reliable migrations/backup, tagged audit events, import safety and query performance.

## Scope discipline

Treat the named subsystems as conceptual boundaries. Build a single Rust library and small application layers, not a speculative plugin ecosystem or cloud services. Prototype the user workflow, measure pain, and introduce abstractions only when actually needed.
