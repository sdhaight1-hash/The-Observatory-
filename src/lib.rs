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
    pub source_version: i64,
    pub source_reference: String,
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
        // Upgrade databases created on the initial development branch.
        let mut stmt = db.prepare("PRAGMA table_info(evidence)")?;
        let columns = stmt.query_map([], |row| row.get::<_, String>(1))?
            .collect::<Result<Vec<_>>>()?;
        drop(stmt);
        if !columns.iter().any(|c| c == "source_version") {
            db.execute_batch("ALTER TABLE evidence ADD COLUMN source_version INTEGER NOT NULL DEFAULT 1;")?;
        }
        if !columns.iter().any(|c| c == "source_reference") {
            db.execute_batch("ALTER TABLE evidence ADD COLUMN source_reference TEXT NOT NULL DEFAULT '';")?;
            // Legacy citations were never pinned to a particular source version.
            // Do not invent a historical reference from the current source address.
            db.execute_batch(
                "UPDATE evidence
                 SET source_reference='UNVERIFIED LEGACY CITATION: original reference unknown',
                     source_version=0;"
            )?;
        }
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
            "INSERT INTO evidence(claim_id,source_id,source_version,source_reference,locator,stance,note)
             SELECT ?1,s.id,s.version,s.reference,?3,?4,?5
             FROM claims c JOIN sources s
             ON s.investigation_id = c.investigation_id
             WHERE c.id=?1 AND s.id=?2",
            params![claim_id, source_id, locator, stance, note],
        )?;
        if added == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(self.db.last_insert_rowid())
    }

    pub fn evidence_for(&self, claim_id: i64) -> Result<Vec<Evidence>> {
        let mut stmt = self.db.prepare(
            "SELECT claim_id,source_id,locator,stance,source_version,source_reference FROM evidence
             WHERE claim_id=?1 ORDER BY id"
        )?;
        let rows = stmt.query_map([claim_id], |row| Ok(Evidence {
            claim_id: row.get(0)?,
            source_id: row.get(1)?,
            locator: row.get(2)?,
            stance: row.get(3)?,
            source_version: row.get(4)?,
            source_reference: row.get(5)?,
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


    /// Generate a human-readable report. Source locators are unverified assertions.
    pub fn report_markdown(&self, investigation_id: i64) -> Result<String> {
        let question: String = self.db.query_row(
            "SELECT question FROM investigations WHERE id=?1",
            [investigation_id], |row| row.get(0)
        )?;
        let mut report = format!("# Investigation: {}\n\n", question);
        let mut stmt = self.db.prepare(
            "SELECT id,kind,statement FROM claims WHERE investigation_id=?1 ORDER BY id"
        )?;
        let claims = stmt.query_map([investigation_id], |row| Ok((
            row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?
        )))?;
        for claim in claims {
            let (id, kind, statement) = claim?;
            report.push_str(&format!("## {}: {}\n\n", kind, statement));
            let mut evidence_stmt = self.db.prepare(
                "SELECT e.stance,s.title,e.source_reference,e.locator,e.note,e.source_version
                 FROM evidence e JOIN sources s ON e.source_id=s.id
                 WHERE e.claim_id=?1 ORDER BY e.id"
            )?;
            let lines = evidence_stmt.query_map([id], |row| Ok((
                row.get::<_, String>(0)?,row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,row.get::<_, i64>(5)?
            )))?;
            for line in lines {
                let (stance,title,reference,locator,note,version) = line?;
                report.push_str(&format!(
                    "- **{}** — {} ({}; captured version {}) at {}. {}\n",
                    stance,title,reference,version,locator,note
                ));
            }
            report.push('\n');
        }
        Ok(report)
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
        let old = o.evidence_for(a)?;
        assert_eq!(old[0].source_version, 1);
        assert_eq!(old[0].source_reference, "local:v1");
        o.link(a, s, "line 3", "context", "")?;
        let new = o.evidence_for(a)?;
        assert_eq!(new[1].source_version, 2);
        assert_eq!(new[1].source_reference, "local:v2");
        let report = o.report_markdown(i)?;
        assert!(report.contains("local:v1"));
        assert!(report.contains("local:v2"));
        Ok(())
    }

    #[test]
    fn rejects_empty_source_revisions_without_changing_history() -> Result<()> {
        let mut o = Observatory::memory()?;
        let i = o.investigation("Q")?;
        let s = o.source(i, "Notebook", "local:original")?;
        assert!(o.revise_source(s, "  ").is_err());
        assert_eq!(o.revision_count(s)?, 0);
        let c = o.claim(i, "Recorded fact", "claim")?;
        o.link(c, s, "line 1", "context", "")?;
        assert_eq!(o.evidence_for(c)?[0].source_reference, "local:original");
        assert_eq!(o.evidence_for(c)?[0].source_version, 1);
        Ok(())
    }

    #[test]
    fn legacy_migration_does_not_invent_historical_citation() -> Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("legacy.sqlite");
        {
            let db = Connection::open(&path)?;
            db.execute_batch(
                "CREATE TABLE investigations (id INTEGER PRIMARY KEY, question TEXT NOT NULL);
                 CREATE TABLE claims (
                   id INTEGER PRIMARY KEY, investigation_id INTEGER NOT NULL,
                   statement TEXT NOT NULL, kind TEXT NOT NULL
                 );
                 CREATE TABLE sources (
                   id INTEGER PRIMARY KEY, investigation_id INTEGER NOT NULL,
                   title TEXT NOT NULL, reference TEXT NOT NULL, version INTEGER NOT NULL DEFAULT 1
                 );
                 CREATE TABLE evidence (
                   id INTEGER PRIMARY KEY, claim_id INTEGER NOT NULL,
                   source_id INTEGER NOT NULL, locator TEXT NOT NULL,
                   stance TEXT NOT NULL, note TEXT NOT NULL DEFAULT ''
                 );
                 INSERT INTO investigations VALUES (1,'Legacy question');
                 INSERT INTO claims VALUES (1,1,'Old claim','claim');
                 INSERT INTO sources VALUES (1,1,'Changed page','local:current',2);
                 INSERT INTO evidence VALUES (1,1,1,'paragraph 4','supports','');"
            )?;
        }
        let migrated = Observatory::open(&path)?;
        let records = migrated.evidence_for(1)?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source_version, 0);
        assert!(records[0].source_reference.contains("UNVERIFIED LEGACY"));
        assert!(!migrated.report_markdown(1)?.contains("(local:current; captured version"));
        drop(migrated);
        // Reopening must not rewrite the migration marker.
        let reopened = Observatory::open(&path)?;
        assert_eq!(reopened.evidence_for(1)?[0].source_version, 0);
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
        let report = reopened.report_markdown(1)?;
        assert!(report.contains("A possibility"));
        assert!(report.contains("file:///notebook"));
        assert!(report.contains("context"));
        Ok(())
    }
}
