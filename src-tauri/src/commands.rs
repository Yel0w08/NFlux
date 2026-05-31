use crate::models::{ApiResponse, CreateNoteRequest, Note, UpdateNoteRequest};
use crate::storage::Storage;
use std::sync::Mutex;
use tauri::State;

/// Get all notes
#[tauri::command]
pub fn get_notes(storage: State<Mutex<Storage>>) -> ApiResponse<Vec<Note>> {
    match storage.lock() {
        Ok(store) => ApiResponse::ok(store.get_all()),
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Get a single note by ID
#[tauri::command]
pub fn get_note(id: String, storage: State<Mutex<Storage>>) -> ApiResponse<Note> {
    match storage.lock() {
        Ok(store) => match store.get(&id) {
            Some(note) => ApiResponse::ok(note),
            None => ApiResponse::err("Note not found".to_string()),
        },
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Create a new note
#[tauri::command]
pub fn create_note(
    request: CreateNoteRequest,
    storage: State<Mutex<Storage>>,
) -> ApiResponse<Note> {
    match storage.lock() {
        Ok(store) => {
            let title = request.title.unwrap_or_default();
            let content = request.content.unwrap_or_default();
            match store.create(title, content) {
                Ok(note) => ApiResponse::ok(note),
                Err(e) => ApiResponse::err(e),
            }
        }
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Update an existing note
#[tauri::command]
pub fn update_note(
    id: String,
    request: UpdateNoteRequest,
    storage: State<Mutex<Storage>>,
) -> ApiResponse<Note> {
    match storage.lock() {
        Ok(store) => match store.update(&id, request) {
            Ok(note) => ApiResponse::ok(note),
            Err(e) => ApiResponse::err(e),
        },
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Delete a note
#[tauri::command]
pub fn delete_note(id: String, storage: State<Mutex<Storage>>) -> ApiResponse<()> {
    match storage.lock() {
        Ok(store) => match store.delete(&id) {
            Ok(_) => ApiResponse::ok(()),
            Err(e) => ApiResponse::err(e),
        },
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Search notes
#[tauri::command]
pub fn search_notes(query: String, storage: State<Mutex<Storage>>) -> ApiResponse<Vec<Note>> {
    match storage.lock() {
        Ok(store) => ApiResponse::ok(store.search(&query)),
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Get notes by archive status
#[tauri::command]
pub fn get_notes_by_status(
    archived: bool,
    storage: State<Mutex<Storage>>,
) -> ApiResponse<Vec<Note>> {
    match storage.lock() {
        Ok(store) => ApiResponse::ok(store.get_by_status(archived)),
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}

/// Get note statistics
#[tauri::command]
pub fn get_stats(storage: State<Mutex<Storage>>) -> ApiResponse<crate::storage::NoteStats> {
    match storage.lock() {
        Ok(store) => ApiResponse::ok(store.get_stats()),
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}
