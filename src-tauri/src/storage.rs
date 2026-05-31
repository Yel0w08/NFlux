use crate::models::{Note, UpdateNoteRequest};
use parking_lot::RwLock;
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

/// Manages note storage and persistence
pub struct Storage {
    notes: Arc<RwLock<Vec<Note>>>,
    data_dir: PathBuf,
}

impl Storage {
    /// Initialize storage with app data directory
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        // Create directory if it doesn't exist
        fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data dir: {}", e))?;

        let mut storage = Storage {
            notes: Arc::new(RwLock::new(Vec::new())),
            data_dir,
        };

        // Load existing notes
        storage.load()?;

        Ok(storage)
    }

    /// Get the path to the notes file
    fn notes_file_path(&self) -> PathBuf {
        self.data_dir.join("notes.json")
    }

    /// Load notes from disk
    fn load(&mut self) -> Result<(), String> {
        let path = self.notes_file_path();

        if !path.exists() {
            return Ok(());
        }

        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read notes file: {}", e))?;
        let notes: Vec<Note> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse notes file: {}", e))?;

        *self.notes.write() = notes;
        Ok(())
    }

    /// Save notes to disk
    pub fn save(&self) -> Result<(), String> {
        let notes = self.notes.read();
        let content = serde_json::to_string_pretty(&*notes)
            .map_err(|e| format!("Failed to serialize notes: {}", e))?;

        let path = self.notes_file_path();
        fs::write(&path, content).map_err(|e| format!("Failed to write notes file: {}", e))?;

        Ok(())
    }

    /// Get all notes
    pub fn get_all(&self) -> Vec<Note> {
        let notes = self.notes.read();
        let mut result = notes.clone();
        result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        result
    }

    /// Get a single note by ID
    pub fn get(&self, id: &str) -> Option<Note> {
        let notes = self.notes.read();
        notes.iter().find(|n| n.id == id).cloned()
    }

    /// Create a new note
    pub fn create(&self, title: String, content: String) -> Result<Note, String> {
        let note = Note::new(title, content);
        {
            let mut notes = self.notes.write();
            notes.insert(0, note.clone());
        }
        self.save()?;
        Ok(note)
    }

    /// Update an existing note
    pub fn update(&self, id: &str, request: UpdateNoteRequest) -> Result<Note, String> {
        let mut notes = self.notes.write();
        let note = notes
            .iter_mut()
            .find(|n| n.id == id)
            .ok_or_else(|| "Note not found".to_string())?;

        if let Some(title) = request.title {
            note.title = title;
        }
        if let Some(content) = request.content {
            note.content = content;
        }
        if let Some(color) = request.color {
            note.color = crate::models::NoteColor::from_string(&color);
        }
        if let Some(archived) = request.archived {
            note.archived = archived;
        }

        note.touch();
        let updated_note = note.clone();
        drop(notes);

        self.save()?;
        Ok(updated_note)
    }

    /// Delete a note
    pub fn delete(&self, id: &str) -> Result<(), String> {
        {
            let mut notes = self.notes.write();
            notes.retain(|n| n.id != id);
        }
        self.save()?;
        Ok(())
    }

    /// Search notes by title and content
    pub fn search(&self, query: &str) -> Vec<Note> {
        let query_lower = query.to_lowercase();
        let notes = self.notes.read();
        let mut results: Vec<Note> = notes
            .iter()
            .filter(|n| {
                n.title.to_lowercase().contains(&query_lower)
                    || n.content.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect();

        results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        results
    }

    /// Get notes by archive status
    pub fn get_by_status(&self, archived: bool) -> Vec<Note> {
        let notes = self.notes.read();
        let mut results: Vec<Note> = notes.iter().filter(|n| n.archived == archived).cloned().collect();
        results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        results
    }

    /// Get note statistics
    pub fn get_stats(&self) -> NoteStats {
        let notes = self.notes.read();
        NoteStats {
            total: notes.len(),
            archived: notes.iter().filter(|n| n.archived).count(),
            active: notes.iter().filter(|n| !n.archived).count(),
        }
    }
}

/// Statistics about notes
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NoteStats {
    pub total: usize,
    pub archived: usize,
    pub active: usize,
}
