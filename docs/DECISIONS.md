# Design Decision Log

This file preserves the project's design reasoning across conversations. All decisions should be evaluated against [FIRST_PRINCIPLES.md](FIRST_PRINCIPLES.md); exceptions need explicit rationale. **Committed** means an agreed architectural/product direction, not implemented code. **Provisional** choices need experimentation. **Deferred** features remain candidates, not obligations.

| ID | Status | Decision | Rationale / boundary |
| --- | --- | --- | --- |
| D-001 | Committed | The Observatory is local-first and investigation-centered. | The useful output is inspectable reasoning, not an opaque AI answer. |
| D-002 | Committed | One canonical knowledge model under multiple views. | Avoid inconsistency between notes, maps, reports and generated articles. |
| D-003 | Committed | Four UI workflows: Discover, Explore, Investigate, Reflect. | Express different cognitive activities without separate silos. |
| D-004 | Committed | Seven conceptual services: Archive, Atlas, Laboratory, Chronicle, Watchtower, Vault, Lens. | Separate responsibilities without creating microservices. |
| D-005 | Committed | AI may propose interpretations; review/acceptance is explicit. | Preserve provenance and prevent silent promotion of fabricated claims. |
| D-006 | Committed | Version-pinned citations and honest uncertainty. | Source changes cannot rewrite earlier evidence; missing historical provenance remains unknown. |
| D-007 | Committed | Rust and SQLite are the small initial technical core. | Local, testable, dependable; add infrastructure only after evidence of need. |
| D-008 | Committed | Manual capture, bookmarks and collections belong in Discover. | Capturing a thought should be low-friction. |
| D-009 | Committed | RSS/Atom and public research feeds belong in Watchtower. | Deliberate, bounded discovery; new items are not accepted evidence automatically. |
| D-010 | Committed | Vault holds archive references, snapshot metadata and content hashes. | Reproducibility and historical source recovery. |
| D-011 | Committed | Associative canvas links differ from typed evidentiary relationships. | A visual line is not proof. |
| D-012 | Committed | Public-interest OSINT methodologies enrich investigation, not define the whole product. | Verification, timelines and source correlation generalize beyond OSINT. |
| D-013 | Committed | The Codex is a derived encyclopedia-like presentation layer. | Readable synthesis with sentence-level evidence paths; not a new truth database. |
| D-014 | Committed | Machine-readable service layer, later authorized MCP adapter. | Assistants and desktop UI share validation; local process is not automatically cloud-accessible. |
| D-015 | Provisional | Tauri + React/TypeScript desktop shell. | Candidate balancing modern UX and Rust integration; prototype before entrenchment. |
| D-016 | Provisional | React Flow/XYFlow for spatial canvas. | Evaluate against scale, layout and manipulation requirements. |
| D-017 | Provisional | OpenAlex, Crossref, Internet Archive/Wayback, Perma.cc as adapters. | Integrate per legal access/API terms and measurable user value. |
| D-018 | Deferred | Automatic Codex generation, prediction calibration and autonomous investigation. | Must follow stable source provenance, review and testing. |
| D-019 | Deferred | HTTP public API, cloud sync, distributed DB, graph DB and plugin marketplace. | Premature deployment and operational complexity. |
| D-020 | Rejected initially | Shadow-library downloading, invasive private-person profiling and unchecked crawling. | Rights, privacy, reliability and security risks; not necessary for mission. |

## Change policy

- Record major decisions with motivation, status and date.
- If a direction changes, document why; do not silently replace previous reasoning.
- Keep **implemented / validated** state in ROADMAP and tests, not in aspirational design language.
- Issue/PR discussions remain the granular engineering record; this log preserves product intent.

## Design review questions

Before accepting a feature ask:
1. Does it help capture, preserve, connect, investigate or reassess knowledge?
2. Does it use the same canonical objects rather than generating a new silo?
3. Can it retain provenance and accurately represent uncertainty?
4. Does it create an unnecessary internet, AI, privacy or legal dependency?
5. What is the smallest end-to-end user workflow that would prove value?
