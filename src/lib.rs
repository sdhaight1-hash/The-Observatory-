//! Small, auditable evidence graph. AI and networking are intentionally absent.
use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct Observatory {
    db: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub claim_id: i64,
    pub source_id: i64,
    pub locator: String,
    pub stance: String,
}

impl Observatory {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Connection::open(path)?;
        Self::init(db)
    }

    pub fn memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(db: Connection) -> Result<Self> {
        db.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS investigations (
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
        Ok(Self { db })
    }

    pub fn investigation(&self, question: &str) -> Result<i64> {
        self.db.execute("INSERT INTO investigations(question) VALUES (?1)", [question])?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn claim(&self, investigation_id: i64, statement: &str, kind: &str) -> Result<i64> {
        self.db.execute(
            "INSERT INTO claims(investigation_id, statement, kind) VALUES (?1,?2,?3)",
            params![investigation_id, statement, kind],
        )?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn source(&self, investigation_id: i64, title: &str, reference: &str) -> Result<i64> {
        self.db.execute(
            "INSERT INTO sources(investigation_id,title,reference) VALUES (?1,?2,?3)",
            params![investigation_id, title, reference],
        )?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn link(&self, claim_id: i64, source_id: i64, locator: &str, stance: &str, note: &str) -> Result<i64> {
        // Claims and sources must belong to the same investigation.
        let added = self.db.execute(
            "INSERT INTO evidence(claim_id,source_id,locator,stance,note)
             SELECT ?1,?2,?3,?4,?5 WHERE EXISTS (
               SELECT 1 FROM claims c JOIN sources s
               ON s.investigation_id = c.investigation_id
               WHERE c.id=?1 AND s.id=?2
             )",
            params![claim_id, source_id, locator, stance, note],
        )?;
        if added == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(self.db.last_insert_rowid())
    }

    pub fn evidence_for(&self, claim_id: i64) -> Result<Vec<Evidence>> {
        let mut stmt = self.db.prepare(
            "SELECT claim_id,source_id,locator,stance FROM evidence
             WHERE claim_id=?1 ORDER BY id"
        )?;
        let rows = stmt.query_map([claim_id], |row| Ok(Evidence {
            claim_id: row.get(0)?,
            source_id: row.get(1)?,
            locator: row.get(2)?,
            stance: row.get(3)?,
        }))?;
        let collected = rows.collect();
        collected
    }

    pub fn dependent_claims(&self, source_id: i64) -> Result<Vec<i64>> {
        let mut stmt = self.db.prepare(
            "SELECT DISTINCT claim_id FROM evidence WHERE source_id=?1 ORDER BY claim_id"
        )?;
        let rows = stmt.query_map([source_id], |row| row.get(0))?;
        let collected = rows.collect();
        collected
    }

    /// Revisions preserve previous references and their dependencies.
    /// Source locations are user-supplied assertions, not automatically verified.
    pub fn revise_source(&mut self, source_id: i64, new_reference: &str) -> Result<()> {
        let tx = self.db.transaction()?;
        let (old_reference, old_version): (String, i64) = tx.query_row(
            "SELECT reference,version FROM sources WHERE id=?1",
            [source_id], |row| Ok((row.get(0)?, row.get(1)?))
        )?;
        tx.execute(
            "UPDATE sources SET reference=?1,version=version+1 WHERE id=?2",
            params![new_reference, source_id],
        )?;
        tx.execute(
            "INSERT INTO revisions(source_id,old_reference,new_reference,old_version,new_version)
             VALUES (?1,?2,?3,?4,?5)",
            params![source_id, old_reference, new_reference, old_version, old_version+1],
        )?;
        tx.commit()
    }

    pub fn revision_count(&self, source_id: i64) -> Result<i64> {
        self.db.query_row(
            "SELECT count(*) FROM revisions WHERE source_id=?1",
            [source_id], |row| row.get(0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_support_and_contradiction_separately() -> Result<()> {
        let o = Observatory::memory()?;
        let i = o.investigation("What explains the observation?")?;
        let c = o.claim(i, "Hypothesis A", "hypothesis")?;
        let s = o.source(i, "Field notes", "file:///notes.txt")?;
        o.link(c, s, "paragraph 2", "supports", "initial observation")?;
        o.link(c, s, "paragraph 5", "contradicts", "counterexample")?;
        let evidence = o.evidence_for(c)?;
        assert_eq!(evidence.len(), 2);
        assert_eq!(evidence[0].stance, "supports");
        assert_eq!(evidence[1].stance, "contradicts");
        Ok(())
    }

    #[test]
    fn rejects_invalid_links_and_missing_locators() -> Result<()> {
        let o = Observatory::memory()?;
        let a = o.investigation("A")?;
        let b = o.investigation("B")?;
        let c = o.claim(a, "A claim", "claim")?;
        let foreign_source = o.source(b, "Source B", "local:B")?;
        assert!(o.link(c, foreign_source, "p.1", "supports", "").is_err());
        let s = o.source(a, "Source A", "local:A")?;
        assert!(o.link(c, s, "", "supports", "").is_err());
        assert!(o.link(c, s, "p.1", "maybe", "").is_err());
        assert!(o.link(999, s, "p.1", "supports", "").is_err());
        Ok(())
    }

    #[test]
    fn revisions_preserve_dependencies_and_history() -> Result<()> {
        let mut o = Observatory::memory()?;
        let i = o.investigation("Q")?;
        let a = o.claim(i, "A", "claim")?;
        let b = o.claim(i, "B", "hypothesis")?;
        let s = o.source(i, "Source", "local:v1")?;
        o.link(a, s, "line 1", "supports", "")?;
        o.link(b, s, "line 2", "contradicts", "")?;
        o.revise_source(s, "local:v2")?;
        assert_eq!(o.dependent_claims(s)?, vec![a, b]);
        assert_eq!(o.revision_count(s)?, 1);
        Ok(())
    }

    #[test]
    fn persists_across_reopen() -> Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("observatory.sqlite");
        let (claim, source);
        {
            let o = Observatory::open(&path)?;
            let i = o.investigation("Persistent question")?;
            claim = o.claim(i, "A possibility", "hypothesis")?;
            source = o.source(i, "Notebook", "file:///notebook")?;
            o.link(claim, source, "line 9", "context", "")?;
        }
        let reopened = Observatory::open(&path)?;
        assert_eq!(reopened.evidence_for(claim)?.len(), 1);
        assert_eq!(reopened.dependent_claims(source)?, vec![claim]);
        Ok(())
    }
}
