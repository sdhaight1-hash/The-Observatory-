//! Explicit, transactional SQLite schema upgrades.
//! Version 0 includes pre-release databases (with or without citation pinning).
use rusqlite::{Connection, Result};

pub const CURRENT_SCHEMA: i64 = 1;

pub fn migrate(db: &mut Connection) -> Result<()> {
    db.pragma_update(None, "foreign_keys", "ON")?;
    let version: i64 = db.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if version > CURRENT_SCHEMA {
        return Err(rusqlite::Error::InvalidQuery);
    }
    if version == CURRENT_SCHEMA {
        return Ok(());
    }

    let tx = db.transaction()?;
    let existing: i64 = tx.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        [],
        |row| row.get(0),
    )?;

    if existing > 0 {
        // Never silently merge our schema into an unrelated or partial database.
        for table in ["investigations", "claims", "sources", "evidence"] {
            let found: i64 = tx.query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get(0),
            )?;
            if found != 1 {
                return Err(rusqlite::Error::InvalidQuery);
            }
        }
    }

    tx.execute_batch(
        "CREATE TABLE IF NOT EXISTS investigations (
             id INTEGER PRIMARY KEY,
             question TEXT NOT NULL CHECK(length(trim(question)) > 0)
         );
         CREATE TABLE IF NOT EXISTS claims (
             id INTEGER PRIMARY KEY,
             investigation_id INTEGER NOT NULL REFERENCES investigations(id),
             statement TEXT NOT NULL CHECK(length(trim(statement)) > 0),
             kind TEXT NOT NULL CHECK(kind IN ('claim','hypothesis'))
         );
         CREATE TABLE IF NOT EXISTS sources (
             id INTEGER PRIMARY KEY,
             investigation_id INTEGER NOT NULL REFERENCES investigations(id),
             title TEXT NOT NULL CHECK(length(trim(title)) > 0),
             reference TEXT NOT NULL CHECK(length(trim(reference)) > 0),
             version INTEGER NOT NULL DEFAULT 1
         );
         CREATE TABLE IF NOT EXISTS evidence (
             id INTEGER PRIMARY KEY,
             claim_id INTEGER NOT NULL REFERENCES claims(id),
             source_id INTEGER NOT NULL REFERENCES sources(id),
             source_version INTEGER NOT NULL DEFAULT 1,
             source_reference TEXT NOT NULL DEFAULT '',
             locator TEXT NOT NULL CHECK(length(trim(locator)) > 0),
             stance TEXT NOT NULL CHECK(stance IN ('supports','contradicts','context')),
             note TEXT NOT NULL DEFAULT ''
         );
         CREATE TABLE IF NOT EXISTS revisions (
             id INTEGER PRIMARY KEY,
             source_id INTEGER NOT NULL REFERENCES sources(id),
             old_reference TEXT NOT NULL,
             new_reference TEXT NOT NULL,
             old_version INTEGER NOT NULL,
             new_version INTEGER NOT NULL,
             changed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE INDEX IF NOT EXISTS evidence_source ON evidence(source_id);
         CREATE INDEX IF NOT EXISTS evidence_claim ON evidence(claim_id);"
    )?;

    let mut stmt = tx.prepare("PRAGMA table_info(evidence)")?;
    let columns = stmt.query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>>>()?;
    drop(stmt);

    if !columns.iter().any(|c| c == "source_version") {
        tx.execute_batch(
            "ALTER TABLE evidence ADD COLUMN source_version INTEGER NOT NULL DEFAULT 1;"
        )?;
    }
    if !columns.iter().any(|c| c == "source_reference") {
        tx.execute_batch(
            "ALTER TABLE evidence ADD COLUMN source_reference TEXT NOT NULL DEFAULT '';"
        )?;
        // A pre-release record's historical source location cannot be inferred
        // from the current source URL. Never fabricate provenance.
        tx.execute_batch(
            "UPDATE evidence
             SET source_version=0,
                 source_reference='UNVERIFIED LEGACY CITATION: original reference unknown';"
        )?;
    }
    tx.pragma_update(None, "user_version", CURRENT_SCHEMA)?;
    tx.commit()
}
