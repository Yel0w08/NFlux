// Main App State
const app = {
    notes: [],
    currentNote: null,
    currentColor: 'white',
    isSearching: false,
    searchQuery: '',
    autoSaveTimer: null,
};

// Color mapping
const colorMap = {
    white: 'white',
    red: 'red',
    orange: 'orange',
    yellow: 'yellow',
    green: 'green',
    teal: 'teal',
    blue: 'blue',
    purple: 'purple',
    pink: 'pink',
    gray: 'gray',
};

// DOM Elements
const elements = {
    notesGrid: document.getElementById('notesGrid'),
    noteModal: document.getElementById('noteModal'),
    optionsModal: document.getElementById('optionsModal'),
    settingsModal: document.getElementById('settingsModal'),
    noteTitle: document.getElementById('noteTitle'),
    noteContent: document.getElementById('noteContent'),
    noteMeta: document.getElementById('noteMeta'),
    newNoteBtn: document.getElementById('newNoteBtn'),
    backBtn: document.getElementById('backBtn'),
    deleteBtn: document.getElementById('deleteBtn'),
    searchBtn: document.getElementById('searchBtn'),
    searchBar: document.getElementById('searchBar'),
    searchInput: document.getElementById('searchInput'),
    closeSearchBtn: document.getElementById('closeSearchBtn'),
    menuBtn: document.getElementById('menuBtn'),
    settingsBtn: document.getElementById('settingsBtn'), // global settings button
    settingsGlobalBtn: document.getElementById('settingsGlobalBtn'), // menu settings
    aboutBtn: document.getElementById('aboutBtn'),
    colorBtns: document.querySelectorAll('.color-btn'), // color buttons in editor footer
    // Settings modal elements
    noteSettingsBtn: document.getElementById('noteSettingsBtn'),
    backFromSettingsBtn: document.getElementById('backFromSettingsBtn'),
    deleteNoteBtn: document.getElementById('deleteNoteBtn'),
    settingsMetadata: document.getElementById('settingsMetadata'),
    settingsColorBtns: document.querySelectorAll('.color-btn-large'),
};

// Initialize app
function init() {
    console.log('App init');
    setupEventListeners();
    loadNotes();
    renderNotes();
}

// Setup event listeners
function setupEventListeners() {
    // New note button
    elements.newNoteBtn.addEventListener('click', createNewNote);

    // Note modal controls
    elements.backBtn.addEventListener('click', closeNoteModal);
    elements.deleteBtn.addEventListener('click', deleteCurrentNote);

    // Search functionality
    elements.searchBtn.addEventListener('click', toggleSearch);
    elements.closeSearchBtn.addEventListener('click', closeSearch);
    elements.searchInput.addEventListener('input', handleSearch);

    // Menu functionality
    elements.menuBtn.addEventListener('click', toggleOptionsMenu);

    // Color selection
    elements.colorBtns.forEach((btn) => {
        btn.addEventListener('click', changeNoteColor);
    });

    // Auto-save on content change
    elements.noteTitle.addEventListener('input', scheduleAutoSave);
    elements.noteContent.addEventListener('input', scheduleAutoSave);

    // Close modals when clicking outside
    elements.noteModal.addEventListener('click', (e) => {
        if (e.target === elements.noteModal) {
            closeNoteModal();
        }
    });

    elements.optionsModal.addEventListener('click', (e) => {
        if (e.target === elements.optionsModal) {
            closeOptionsMenu();
        }
    });

    // Keyboard shortcuts
    document.addEventListener('keydown', handleKeyboardShortcuts);

    // Preview toggle
    const previewToggleBtn = document.getElementById('previewToggleBtn');
    if (previewToggleBtn) {
        previewToggleBtn.addEventListener('click', togglePreview);
    }
}

// Note Management Functions
async function createNewNote() {
    console.log('Create New Note clicked');
        const newNote = {
        id: generateId(),
        title: '',
        content: '',
        color: 'white',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
    };

    app.currentNote = newNote;
    app.notes.push(newNote);
    app.currentColor = 'white';
    openNoteModal();
    focusNoteTitle();
}

function openNoteModal() {
    if (!app.currentNote) return;

    elements.noteTitle.value = app.currentNote.title || '';
    elements.noteContent.value = app.currentNote.content || '';
    app.currentColor = app.currentNote.color || 'white';

    updateNoteMeta();
    updateColorSelection();
    elements.noteModal.classList.add('active');

    // Apply background color to modal
    applyNoteColor();

    // Disable body scroll
    document.body.style.overflow = 'hidden';
}

function closeNoteModal() {
    if (app.currentNote) {
        saveCurrentNote();
    }

    elements.noteModal.classList.remove('active');
    app.currentNote = null;
    document.body.style.overflow = '';
    renderNotes();
}

async function saveCurrentNote() {
    if (!app.currentNote) return;

    app.currentNote.title = elements.noteTitle.value.trim();
    app.currentNote.content = elements.noteContent.value.trim();
    app.currentNote.color = app.currentColor;
    app.currentNote.updatedAt = new Date().toISOString();

    // Call Tauri API to save
    try {
        await window.__TAURI__.invoke('update_note', app.currentNote);
    } catch (error) {
        console.error('Failed to save note:', error);
    }
}

async function deleteCurrentNote() {
    if (!app.currentNote || !confirm('Delete this note?')) return;

    const noteId = app.currentNote.id;
    app.notes = app.notes.filter((note) => note.id !== noteId);

    try {
        await window.__TAURI__.invoke('delete_note', { id: noteId });
    } catch (error) {
        console.error('Failed to delete note:', error);
    }

    closeNoteModal();
}

function scheduleAutoSave() {
    clearTimeout(app.autoSaveTimer);
    app.autoSaveTimer = setTimeout(saveCurrentNote, 1000);
}

function focusNoteTitle() {
    setTimeout(() => elements.noteTitle.focus(), 100);
}

function updateNoteMeta() {
    if (!app.currentNote) return;

    const updated = new Date(app.currentNote.updatedAt);
    const today = new Date();
    const isToday = updated.toDateString() === today.toDateString();

    let dateStr;
    if (isToday) {
        dateStr = updated.toLocaleTimeString('en-US', {
            hour: 'numeric',
            minute: '2-digit',
        });
    } else {
        dateStr = updated.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: updated.getFullYear() !== today.getFullYear() ? 'numeric' : undefined,
        });
    }

    elements.noteMeta.textContent = `Edited ${dateStr}`;
}

// Search functionality
function toggleSearch() {
    app.isSearching = !app.isSearching;

    if (app.isSearching) {
        elements.searchBar.style.display = 'flex';
        setTimeout(() => elements.searchInput.focus(), 100);
    } else {
        closeSearch();
    }
}

function closeSearch() {
    app.isSearching = false;
    app.searchQuery = '';
    elements.searchBar.style.display = 'none';
    elements.searchInput.value = '';
    renderNotes();
}

function handleSearch() {
    app.searchQuery = elements.searchInput.value.toLowerCase();
    renderNotes();
}

// Color management
function changeNoteColor(e) {
    const colorBtn = e.currentTarget;
    app.currentColor = colorBtn.dataset.color;
    updateColorSelection();
    applyNoteColor();
}

function updateColorSelection() {
    elements.colorBtns.forEach((btn) => {
        if (btn.dataset.color === app.currentColor) {
            btn.classList.add('active');
        } else {
            btn.classList.remove('active');
        }
    });
}

function applyNoteColor() {
    const modalContent = elements.noteModal.querySelector('.modal-content');
    if (!modalContent) return;

    // Remove all color classes
    Object.keys(colorMap).forEach((color) => {
        modalContent.classList.remove(color);
    });

    // Add current color class
    if (app.currentColor !== 'white') {
        modalContent.classList.add(app.currentColor);
    }
}

// Rendering
function renderNotes() {
    let notesToDisplay = app.notes;

    // Filter based on search
    if (app.searchQuery) {
        notesToDisplay = notesToDisplay.filter((note) => {
            const title = note.title.toLowerCase();
            const content = note.content.toLowerCase();
            return (
                title.includes(app.searchQuery) ||
                content.includes(app.searchQuery)
            );
        });
    }

    // Sort by updated date (most recent first)
    notesToDisplay.sort(
        (a, b) =>
            new Date(b.updatedAt).getTime() -
            new Date(a.updatedAt).getTime()
    );

    // Clear grid
    elements.notesGrid.innerHTML = '';

    // Show empty state
    if (notesToDisplay.length === 0) {
        if (app.searchQuery) {
            elements.notesGrid.innerHTML = `
                <div class="empty-state">
                    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                        <circle cx="11" cy="11" r="8"></circle>
                        <path d="m21 21-4.35-4.35"></path>
                    </svg>
                    <p>No notes found</p>
                    <small>"${app.searchQuery}" not found in any notes</small>
                </div>
            `;
        } else {
            elements.notesGrid.innerHTML = `
                <div class="empty-state">
                    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                        <rect x="3" y="3" width="18" height="18" rx="2"></rect>
                        <line x1="9" y1="9" x2="15" y2="9"></line>
                        <line x1="9" y1="15" x2="15" y2="15"></line>
                    </svg>
                    <p>No notes yet</p>
                    <small>Tap the button below to create your first note</small>
                </div>
            `;
        }
        return;
    }

    // Render note cards
    notesToDisplay.forEach((note) => {
        const card = createNoteCard(note);
        elements.notesGrid.appendChild(card);
    });
}

function createNoteCard(note) {
    const card = document.createElement('div');
    card.className = `note-card ${note.color}`;
    card.style.cursor = 'pointer';

    const preview = note.content || note.title || 'Empty note';
    const lines = preview.split('\n').slice(0, 3).join('\n');

    card.innerHTML = `
        ${note.title ? `<div class="note-card-title">${escapeHtml(note.title)}</div>` : ''}
        <div class="note-card-content">${escapeHtml(lines)}</div>
        <div class="note-card-date">${formatDate(note.updatedAt)}</div>
    `;

    card.addEventListener('click', () => {
        app.currentNote = note;
        openNoteModal();
    });

    return card;
}

function formatDate(dateString) {
    const date = new Date(dateString);
    const today = new Date();
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    if (date.toDateString() === today.toDateString()) {
        return date.toLocaleTimeString('en-US', {
            hour: 'numeric',
            minute: '2-digit',
        });
    } else if (date.toDateString() === yesterday.toDateString()) {
        return 'Yesterday';
    } else if (date.getFullYear() === today.getFullYear()) {
        return date.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
        });
    } else {
        return date.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: '2-digit',
        });
    }
}

// Data management
async function loadNotes() {
    try {
        app.notes = await window.__TAURI__.invoke('get_notes');
    } catch (error) {
        console.error('Failed to load notes:', error);
        app.notes = [];
    }
}

// Menu functionality
function toggleOptionsMenu() {
    elements.optionsModal.classList.toggle('active');
}

function closeOptionsMenu() {
    elements.optionsModal.classList.remove('active');
}

function showSettings() {
    alert('Settings feature coming soon!');
    closeOptionsMenu();
}

function showAbout() {
    alert('NFlux Notes v1.0\n\nA modern notes app built with Tauri and Rust.\n\nDesigned for productivity and simplicity.');
    closeOptionsMenu();
}

// Keyboard shortcuts
function handleKeyboardShortcuts(e) {
    // Ctrl/Cmd + N: New note
    if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
        e.preventDefault();
        createNewNote();
    }

    // Escape: Close modal or search
    if (e.key === 'Escape') {
        if (elements.noteModal.classList.contains('active')) {
            closeNoteModal();
        } else if (app.isSearching) {
            closeSearch();
        }
    }

    // Ctrl/Cmd + F: Search
    if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
        e.preventDefault();
        toggleSearch();
    }
}

// Utility functions
function generateId() {
    return `note_${Date.now()}_${Math.random()
        .toString(36)
        .substr(2, 9)}`;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Initialize on DOM ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Handle app lifecycle
window.addEventListener('beforeunload', () => {
    if (app.currentNote) {
        saveCurrentNote();
    }
});
