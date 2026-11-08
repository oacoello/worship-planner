use crate::models::{
    AppSettings, Arrangement, ItemType, Language, SermonSeries, ServiceItem, ServicePlan, Song,
    ThemeMode,
};
use directories::ProjectDirs;
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_default() -> Result<Self> {
        let db_path = Self::default_db_path();
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn default_db_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "grancoplanner", "GrancoPlanner") {
            proj_dirs.data_local_dir().join("service_planner.db")
        } else {
            PathBuf::from("service_planner.db")
        }
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS series (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                artwork TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS services (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                series_id TEXT NOT NULL DEFAULT '',
                series TEXT NOT NULL DEFAULT '',
                sermon_title TEXT NOT NULL DEFAULT '',
                week_number TEXT NOT NULL DEFAULT '',
                sermon_art TEXT,
                date TEXT NOT NULL,
                time TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                service_id TEXT NOT NULL,
                item_type TEXT NOT NULL,
                title TEXT NOT NULL,
                subtitle TEXT NOT NULL DEFAULT '',
                arrangement_name TEXT NOT NULL DEFAULT '',
                key TEXT NOT NULL DEFAULT '',
                lead_vocalist TEXT NOT NULL DEFAULT '',
                duration INTEGER NOT NULL DEFAULT 0,
                sort_order INTEGER NOT NULL DEFAULT 0,
                notes TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS songs (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                artist TEXT NOT NULL DEFAULT '',
                arrangements_json TEXT NOT NULL
            );
            ",
        )
    }

    // =======================
    // SETTINGS
    // =======================
    pub fn get_settings(&self) -> AppSettings {
        let mut settings = AppSettings::default();
        if let Ok(mut stmt) = self.conn.prepare("SELECT key, value FROM settings") {
            if let Ok(rows) = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            }) {
                for r in rows.flatten() {
                    match r.0.as_str() {
                        "app_language" => settings.language = Language::from_code(&r.1),
                        "app_theme_mode" => settings.theme_mode = ThemeMode::from_str(&r.1),
                        _ => {}
                    }
                }
            }
        }
        settings
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_language', ?1)",
            params![settings.language.as_code()],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_theme_mode', ?1)",
            params![settings.theme_mode.as_str()],
        )?;
        Ok(())
    }

    // =======================
    // SERMON SERIES
    // =======================
    pub fn get_sermon_series(&self) -> Result<Vec<SermonSeries>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, artwork, created_at FROM series ORDER BY LOWER(title) ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(SermonSeries {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                artwork: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    #[allow(dead_code)]
    pub fn get_sermon_series_by_id(&self, id: &str) -> Result<Option<SermonSeries>> {
        if id.is_empty() {
            return Ok(None);
        }
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, artwork, created_at FROM series WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(SermonSeries {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                artwork: row.get(3)?,
                created_at: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn save_sermon_series(&self, mut series: SermonSeries) -> Result<SermonSeries> {
        if series.id.is_empty() {
            series.id = Uuid::new_v4().to_string();
        }
        if series.created_at.is_empty() {
            series.created_at = chrono::Utc::now().to_rfc3339();
        }
        self.conn.execute(
            "INSERT OR REPLACE INTO series (id, title, description, artwork, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                series.id,
                series.title,
                series.description,
                series.artwork,
                series.created_at
            ],
        )?;
        Ok(series)
    }

    pub fn delete_sermon_series(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM series WHERE id = ?1", params![id])?;
        Ok(())
    }

    // =======================
    // SERVICES
    // =======================
    pub fn get_services(&self) -> Result<Vec<ServicePlan>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, series_id, series, sermon_title, week_number, sermon_art, date, time
             FROM services
             ORDER BY date ASC, time ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ServicePlan {
                id: row.get(0)?,
                title: row.get(1)?,
                series_id: row.get(2)?,
                series: row.get(3)?,
                sermon_title: row.get(4)?,
                week_number: row.get(5)?,
                sermon_art: row.get(6)?,
                date: row.get(7)?,
                time: row.get(8)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    #[allow(dead_code)]
    pub fn get_service(&self, id: &str) -> Result<Option<ServicePlan>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, series_id, series, sermon_title, week_number, sermon_art, date, time
             FROM services WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(ServicePlan {
                id: row.get(0)?,
                title: row.get(1)?,
                series_id: row.get(2)?,
                series: row.get(3)?,
                sermon_title: row.get(4)?,
                week_number: row.get(5)?,
                sermon_art: row.get(6)?,
                date: row.get(7)?,
                time: row.get(8)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn save_service(&self, mut service: ServicePlan) -> Result<ServicePlan> {
        if service.id.is_empty() {
            service.id = Uuid::new_v4().to_string();
        }
        self.conn.execute(
            "INSERT OR REPLACE INTO services
             (id, title, series_id, series, sermon_title, week_number, sermon_art, date, time)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                service.id,
                service.title,
                service.series_id,
                service.series,
                service.sermon_title,
                service.week_number,
                service.sermon_art,
                service.date,
                service.time,
            ],
        )?;
        Ok(service)
    }

    pub fn delete_service(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM services WHERE id = ?1", params![id])?;
        self.conn
            .execute("DELETE FROM items WHERE service_id = ?1", params![id])?;
        Ok(())
    }

    // =======================
    // SERVICE ITEMS
    // =======================
    pub fn get_all_items(&self) -> Result<Vec<ServiceItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, service_id, item_type, title, subtitle, arrangement_name, key, lead_vocalist, duration, sort_order, notes
             FROM items
             ORDER BY sort_order ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let type_str: String = row.get(2)?;
            Ok(ServiceItem {
                id: row.get(0)?,
                service_id: row.get(1)?,
                item_type: ItemType::from_str(&type_str),
                title: row.get(3)?,
                subtitle: row.get(4)?,
                arrangement_name: row.get(5)?,
                key: row.get(6)?,
                lead_vocalist: row.get(7)?,
                duration: row.get(8)?,
                order: row.get(9)?,
                notes: row.get(10)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn get_items_for_service(&self, service_id: &str) -> Result<Vec<ServiceItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, service_id, item_type, title, subtitle, arrangement_name, key, lead_vocalist, duration, sort_order, notes
             FROM items
             WHERE service_id = ?1
             ORDER BY sort_order ASC",
        )?;
        let rows = stmt.query_map(params![service_id], |row| {
            let type_str: String = row.get(2)?;
            Ok(ServiceItem {
                id: row.get(0)?,
                service_id: row.get(1)?,
                item_type: ItemType::from_str(&type_str),
                title: row.get(3)?,
                subtitle: row.get(4)?,
                arrangement_name: row.get(5)?,
                key: row.get(6)?,
                lead_vocalist: row.get(7)?,
                duration: row.get(8)?,
                order: row.get(9)?,
                notes: row.get(10)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn save_item(&self, mut item: ServiceItem) -> Result<ServiceItem> {
        if item.id.is_empty() {
            item.id = Uuid::new_v4().to_string();
        }
        self.conn.execute(
            "INSERT OR REPLACE INTO items
             (id, service_id, item_type, title, subtitle, arrangement_name, key, lead_vocalist, duration, sort_order, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                item.id,
                item.service_id,
                item.item_type.as_str(),
                item.title,
                item.subtitle,
                item.arrangement_name,
                item.key,
                item.lead_vocalist,
                item.duration,
                item.order,
                item.notes,
            ],
        )?;
        Ok(item)
    }

    pub fn delete_item(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM items WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn save_items(&self, items: &[ServiceItem]) -> Result<()> {
        for item in items {
            let _ = self.save_item(item.clone())?;
        }
        Ok(())
    }

    // =======================
    // SONG BANK & ARRANGEMENTS
    // =======================
    pub fn get_songs(&self) -> Result<Vec<Song>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, artist, arrangements_json FROM songs ORDER BY LOWER(title) ASC")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let artist: String = row.get(2)?;
            let arr_json: String = row.get(3)?;
            let arrangements: Vec<Arrangement> =
                serde_json::from_str(&arr_json).unwrap_or_default();
            Ok(Song {
                id,
                title,
                artist,
                arrangements,
            }
            .normalized())
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn save_song(&self, mut song: Song) -> Result<Song> {
        if song.id.is_empty() {
            song.id = Uuid::new_v4().to_string();
        }
        song = song.normalized();
        let arr_json = serde_json::to_string(&song.arrangements).unwrap_or_else(|_| "[]".into());
        self.conn.execute(
            "INSERT OR REPLACE INTO songs (id, title, artist, arrangements_json)
             VALUES (?1, ?2, ?3, ?4)",
            params![song.id, song.title, song.artist, arr_json],
        )?;
        Ok(song)
    }

    pub fn delete_song(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM songs WHERE id = ?1", params![id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_crud_operations() {
        let db = Database::open_in_memory().expect("in-memory db");

        // 1. Settings
        let mut settings = db.get_settings();
        assert_eq!(settings.language, Language::Es);
        settings.language = Language::En;
        settings.theme_mode = ThemeMode::Light;
        db.save_settings(&settings).unwrap();
        let loaded_settings = db.get_settings();
        assert_eq!(loaded_settings.language, Language::En);
        assert_eq!(loaded_settings.theme_mode, ThemeMode::Light);

        // 2. Sermon Series
        let series = db
            .save_sermon_series(SermonSeries::new(
                "Grace Upon Grace".into(),
                "Book of Romans".into(),
                None,
            ))
            .unwrap();
        assert!(!series.id.is_empty());
        assert_eq!(db.get_sermon_series().unwrap().len(), 1);
        assert!(db.get_sermon_series_by_id(&series.id).unwrap().is_some());

        // 3. Songs & Arrangements
        let song = db
            .save_song(Song {
                id: String::new(),
                title: "Living Hope".into(),
                artist: "Phil Wickham".into(),
                arrangements: vec![Arrangement {
                    id: "arr1".into(),
                    name: "Standard".into(),
                    key: "G".into(),
                    bpm: 72,
                    duration: 310,
                }],
            })
            .unwrap();
        assert_eq!(db.get_songs().unwrap().len(), 1);
        assert_eq!(song.key_summary(), "G");

        // 4. Service & Items + Cascading delete
        let svc = db
            .save_service(ServicePlan {
                id: String::new(),
                title: "Sunday Morning Worship".into(),
                series_id: series.id.clone(),
                series: series.title.clone(),
                sermon_title: "Walking in Freedom".into(),
                week_number: "Week 1".into(),
                sermon_art: None,
                date: "2026-11-08".into(),
                time: "10:00".into(),
            })
            .unwrap();

        let _item1 = db
            .save_item(ServiceItem {
                id: String::new(),
                service_id: svc.id.clone(),
                item_type: ItemType::Song,
                title: "Living Hope".into(),
                subtitle: "Phil Wickham".into(),
                arrangement_name: "Standard".into(),
                key: "G".into(),
                lead_vocalist: "Sarah M.".into(),
                duration: 310,
                order: 0,
                notes: "Acoustic intro".into(),
            })
            .unwrap();

        assert_eq!(db.get_items_for_service(&svc.id).unwrap().len(), 1);

        db.delete_service(&svc.id).unwrap();
        assert!(db.get_service(&svc.id).unwrap().is_none());
        assert_eq!(db.get_items_for_service(&svc.id).unwrap().len(), 0);
    }
}
