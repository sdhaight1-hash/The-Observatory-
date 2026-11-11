# The Observatory — Product Vision

**Version:** 2.0 working vision  
**Recorded:** 2026-11-11  
**Status:** Direction approved for planning; not a claim that these features exist.

## North star

**Discover freely. Preserve faithfully. Connect meaningfully. Investigate rigorously. Revisit honestly.**

The Observatory is a **local-first knowledge exploration and investigation environment**. It connects frictionless capture and personal knowledge management to inspectable evidence, hypotheses, provenance, and revisions. It should support learning, scientific research, history, public-interest OSINT, and other evidence-driven inquiries without being constrained to any one domain.

It is **not** primarily a chatbot, generic notes app, web browser, general office suite, surveillance platform, or automated encyclopedia. Its distinguishing capability is exposing *how a conclusion was formed, which evidence it depends on, and what could change it.*

## User journey

1. **Encounter** an idea, publication, URL, image, observation, or dataset.
2. **Capture** it immediately without organizing everything up front.
3. **Discover** related material from deliberately selected sources.
4. **Explore** it through notes, collections, spatial canvases, graphs, tables, and timelines.
5. **Investigate** specific questions through competing hypotheses, claims, supporting/contradicting evidence, and explicit unknowns.
6. **Preserve** source identity, precise cited passages, capture times, snapshots when lawful, and change history.
7. **Reflect** on evolving conclusions and earlier predictions; export reproducible findings.
8. **Collaborate** with authorized assistants via a machine interface, with proposed AI assertions separated from accepted evidence.

An article may be saved in the inbox, placed on several canvases, cited in an investigation, and later surfaced in a generated knowledge article. Those are **views and relationships to one resource**, not disconnected copies.

## Four primary user workspaces

- **Discover** — quick capture, bookmarks, RSS/Atom subscriptions, discovery inbox, tagging, collections, search, and suggestions.
- **Explore** — reusable cards, mind maps, infinite canvas, backlinks, knowledge graphs, list/table views, and semantic relationships.
- **Investigate** — source-backed claims, hypotheses, typed evidence links, contradiction inspection, entity resolution, public-interest OSINT, and testable analyses.
- **Reflect** — chronological activity, investigation history, changes of mind, prediction tracking, and review of unresolved questions.

These are navigation/workflow surfaces, not separate databases.

## Seven conceptual services

| Service | Responsibility |
| --- | --- |
| **Archive** | Documents, notes, media, datasets, metadata and search |
| **Atlas** | Entities, typed relationships, graphs and spatial views |
| **Laboratory** | Claims, evidence, hypotheses, tests and assessments |
| **Chronicle** | Source revisions, research history, timelines and prediction records |
| **Watchtower** | Bookmark capture, RSS/Atom, selected research feeds and change discovery |
| **Vault** | Source versions, snapshots, archive links, hashes and recoverability |
| **Lens** | Human/AI interaction, search, contextual analysis and investigation tools |

These are modular responsibilities inside one application, **not** seven microservices.

## The Codex — a future presentation layer

Inspired in part by the *idea* of AI-maintained encyclopedic writing (including Grokipedia), the **Codex** will eventually assemble readable knowledge articles from the user's Library and accepted investigations.

- Every substantive sentence should link to supporting recorded claims/evidence where available.
- Clearly separate sourced findings, provisional synthesis, AI-generated suggestions, and unresolved disputes.
- Allow direct drill-down from a sentence to passage, version, relationship, and counterevidence.
- Show article changes over time; never silently overwrite the earlier explanation.
- An article is a **derived view**, not a second, independent truth database.
- No claim that an AI-generated encyclopedia entry is inherently accurate.

## Inspirations, not cloning targets

- **mymind:** near-zero-friction capture, flexible organization, visual browsing.
- **Notion:** relational structured records and multiple consistent views.
- **Obsidian:** links, backlinks, local-first notes and knowledge graphs.
- **Heptabase:** spatial thinking, whiteboards and reusable research cards.
- **Raindrop.io:** useful bookmark metadata and collections.
- **Feedly:** deliberate RSS/feed subscriptions.
- **Wayback Machine / Internet Archive / Perma.cc:** archival lookup, source history, citation longevity; use permitted APIs and obey access/reuse terms.
- **Maltego / Bellingcat / OpenCTI:** link analysis, verification discipline, and typed relationships; do not inherit cyber-specific or surveillance-centric requirements.
- **Grokipedia:** readable living knowledge pages, while explicitly rejecting opaque sourcing and silent AI authority.

## Differentiation

The product hypothesis is that **one traceable research graph with multiple user-facing views** can support both informal exploration and formal evidence analysis better than a disconnected set of bookmarks, notebooks, diagrams and generated answers.

Test this with actual workflows, not visual polish alone. A visually impressive network does not validate its relationships.

## Limits and ethics

- Legitimate research of public information is in scope; indiscriminate personal-data aggregation, invasive profiling, harassment and mass surveillance are out of scope.
- No unlicensed bulk downloading, copyright circumvention, or shadow-library acquisition workflow. Bibliographic metadata may be considered where legally and operationally appropriate.
- Archive capture must respect legal access and rights; a stored page is not proof its contents were true.
- AI suggestions require explicit provenance and review. No silent promotion of guesses into accepted conclusions.
- Data remains local by default; network access is explicit, scoped, and auditable.
- No mandatory subscriptions, cloud sync, LLM, server deployment, or network access for core research.

## How to use this document

Read [ARCHITECTURE.md](ARCHITECTURE.md) for boundaries and data model, [ROADMAP.md](ROADMAP.md) for build order and acceptance tests, and [DECISIONS.md](DECISIONS.md) for committed, provisional, deferred, and rejected choices. These documents deliberately distinguish **vision** from **implemented functionality**.
