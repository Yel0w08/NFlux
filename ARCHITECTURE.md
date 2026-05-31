# NFlux - Professional Markdown Note Editor

A desktop note-taking application built with **Tauri 2** (Rust + WebView) and modern class-based JavaScript architecture. Features a beautiful dark UI, full markdown support, live preview, and two separate pages for note selection and editing.

## 🏗️ Architecture Overview

### Frontend (JavaScript - Class-Based)
- **API.js** - Tauri command handler with error management
- **Router.js** - Page navigation system with state management
- **EventEmitter.js** - Global event bus for component communication
- **NotesPage.js** - Notes list selector component
- **EditorPage.js** - Full-featured markdown editor component

### Backend (Rust - Tauri)
- **models.rs** - Data models (Note, NoteColor, ApiResponse)
- **storage.rs** - File-based persistence with atomic operations
- **commands.rs** - Tauri command handlers
- **lib.rs** - Application initialization and setup

## ✨ Features

### Core Functionality
- ✅ **Markdown Editor** - Full CommonMark support with real-time preview
- ✅ **Two-Page Layout** - Separate notes selector and editor pages
- ✅ **Syntax Highlighting** - Code blocks with Highlight.js
- ✅ **Live Preview** - Split-screen markdown rendering
- ✅ **Color Tags** - 8 color options for note organization
- ✅ **Archive System** - Hide notes without deletion
- ✅ **Full-Text Search** - Search across titles and content
- ✅ **Auto-Save** - 500ms debounce saves automatically
- ✅ **Persistent Storage** - File-based JSON storage
- ✅ **Statistics** - Word/character counts and note stats

### Toolbar
- Bold, Italic, Strikethrough text
- Headings (H1, H2, H3)
- Bullet and numbered lists
- Code blocks with syntax highlighting
- Blockquotes and links
- Preview toggle

### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Ctrl+N` / `Cmd+N` | New note (Notes page) |
| `Ctrl+B` / `Cmd+B` | Bold text |
| `Ctrl+I` / `Cmd+I` | Italic text |
| `Ctrl+K` / `Cmd+K` | Insert link |
| `Ctrl+S` / `Cmd+S` | Save note |
| `Escape` | Go back to notes |
| `↑/↓` | Navigate notes |

## 📁 Project Structure

```
nflux/
├── src/                          # Frontend (HTML, CSS, JavaScript)
│   ├── index.html               # Main entry point
│   ├── styles.css               # Complete styling
│   ├── main.js                  # Initialization
│   └── classes/                 # JavaScript classes
│       ├── API.js              # Tauri command handler
│       ├── Router.js           # Page router
│       ├── EventEmitter.js     # Event system
│       ├── NotesPage.js        # Notes selector
│       └── EditorPage.js       # Editor component
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs              # App initialization
│   │   ├── main.rs             # Entry point
│   │   ├── models.rs           # Data models
│   │   ├── storage.rs          # File persistence
│   │   └── commands.rs         # Tauri handlers
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
└── README.md
```

## 🚀 Getting Started

### Requirements
- Node.js 18+
- Rust 1.70+
- Tauri CLI 2

### Installation

```bash
# Install dependencies
npm install
cd src-tauri && cargo fetch

# Development
npm run tauri dev

# Build
npm run tauri build

# Check Rust
cd src-tauri && cargo check
```

## 📊 Data Model

### Note Structure
```typescript
interface Note {
  id: string;                    // UUID v4
  title: string;                 // Note title
  content: string;               // Markdown content
  color: "default" | "red" | "orange" | "yellow" | "green" | "teal" | "blue" | "purple";
  archived: boolean;             // Archive flag
  created_at: DateTime<Utc>;    // ISO 8601 timestamp
  updated_at: DateTime<Utc>;    // ISO 8601 timestamp
}
```

### API Response
```typescript
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

## 🔌 Tauri Commands

### Notes Management
- `get_notes()` - Get all active notes
- `get_note(id)` - Get single note
- `create_note(request)` - Create new note
- `update_note(id, request)` - Update note
- `delete_note(id)` - Delete note
- `search_notes(query)` - Search notes
- `get_notes_by_status(archived)` - Filter by archive status
- `get_stats()` - Get statistics

## 🏗️ Class Architecture

### API Class
Singleton for all Tauri commands with error handling.
```javascript
API.getNotes()
API.createNote(title, content)
API.updateNote(id, request)
API.deleteNote(id)
```

### Router Class
Manages page navigation with lifecycle hooks.
```javascript
router.register(name, page)
router.navigate(page, params)
router.back()
router.isActive(page)
```

### EventEmitter Class
Global event bus for component communication.
```javascript
eventBus.on(event, callback)
eventBus.emit(event, ...args)
eventBus.off(event, callback)
```

### NotesPage Class
Notes list selector with search and filtering.
- `loadNotes()` - Load from backend
- `selectNote(id)` - Navigate to editor
- `createNewNote()` - Create and open editor
- `setFilter(filter)` - Apply filter
- `toggleSearch()` - Show/hide search

### EditorPage Class
Full-featured markdown editor.
- `loadNote(id)` - Load note content
- `saveNote()` - Persist changes
- `applyFormat(type)` - Apply text formatting
- `updatePreview()` - Update markdown preview
- `togglePreview()` - Show/hide preview

## 💾 Storage

Notes are stored in JSON format at:
- **Windows**: `%APPDATA%\nflux\data\notes.json`
- **macOS**: `~/Library/Application Support/nflux/notes.json`
- **Linux**: `~/.config/nflux/notes.json`

## 🎨 UI/UX

### Theme
- Dark theme (dark-first design)
- Accent color: Purple (#a89ff9)
- Support for 8 note colors
- Smooth animations and transitions

### Responsive
- Desktop: Full layout with sidebar + editor
- Tablet: Adjusted spacing
- Mobile: Stacked layout

## 📦 Dependencies

### Frontend
- `marked` - Markdown parser
- `highlight.js` - Syntax highlighting
- Tauri API

### Backend
- `tauri` - Desktop framework
- `serde` - Serialization
- `chrono` - Date/time
- `uuid` - ID generation
- `parking_lot` - Fast mutex
- `tokio` - Async runtime

## 🔒 Security

- ✅ All notes stored locally (no cloud)
- ✅ File-based storage with JSON serialization
- ✅ No external API calls
- ✅ Input validation on all commands
- ✅ Safe concurrent access with RwLock/Mutex

## 🗺️ Roadmap

- [ ] Export to PDF
- [ ] Export to Markdown files
- [ ] Tag system
- [ ] Note encryption
- [ ] Cloud sync
- [ ] Collaborative editing
- [ ] Custom themes
- [ ] Spell checker
- [ ] Voice notes
- [ ] Mobile app

## 📝 Development Notes

### Adding New Features

1. **Backend (Rust)**
   - Add command in `commands.rs`
   - Update `storage.rs` if needed
   - Test with `cargo check`

2. **Frontend (JS)**
   - Add method to `API` class
   - Use in page components
   - Handle with EventEmitter

3. **UI (CSS)**
   - Add styles to `styles.css`
   - Follow dark theme conventions
   - Test responsiveness

## 🐛 Troubleshooting

### Compilation Issues
```bash
# Clean build
cd src-tauri && cargo clean && cargo check

# Update dependencies
cd src-tauri && cargo update
```

### Storage Issues
- Check file permissions
- Verify `notes.json` exists
- Check app data directory

### UI Issues
- Clear browser cache
- Check console for errors
- Verify module imports

## 📄 License

See LICENSE file

## 🤝 Contributing

1. Fork the project
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 💬 Support

For issues, questions, or suggestions:
- Open an issue on GitHub
- Check existing documentation
- Review code comments

---

**Built with ❤️ using Tauri & Rust**
