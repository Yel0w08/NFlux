# NFlux Development Guide

## 📚 Quick Navigation

- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Adding New Features](#adding-new-features)
- [Code Standards](#code-standards)
- [Common Tasks](#common-tasks)
- [Troubleshooting](#troubleshooting)

## 🗂️ Project Structure

### Frontend (`src/`)
```
src/
├── index.html              # Single HTML file with both pages
├── styles.css             # All styles (900+ lines, well-organized)
├── main.js                # Initialization placeholder
└── classes/               # Class-based components
    ├── API.js            # Tauri command handler
    ├── Router.js         # Page navigation (NotesPage ↔ EditorPage)
    ├── EventEmitter.js   # Global event bus
    ├── NotesPage.js      # Notes selector (400+ lines)
    └── EditorPage.js     # Markdown editor (500+ lines)
```

### Backend (`src-tauri/src/`)
```
src-tauri/src/
├── lib.rs                 # App setup & Tauri initialization
├── main.rs                # Entry point (minimal)
├── models.rs              # Data structures (200+ lines)
├── storage.rs             # File persistence (250+ lines)
└── commands.rs            # Tauri commands (100+ lines)
```

## 🚀 Getting Started

### Prerequisites
```bash
# Node.js
node --version  # Should be v18+

# Rust
rustc --version # Should be recent stable

# Tauri CLI (optional, npm scripts work too)
cargo install create-tauri-app
```

### Initial Setup
```bash
# 1. Clone/navigate to project
cd NFlux

# 2. Install dependencies
npm install
cd src-tauri && cargo fetch && cd ..

# 3. Verify build
cd src-tauri && cargo check && cd ..

# 4. Start development
npm run tauri dev
```

## 💻 Development Workflow

### Working on Frontend

#### Adding a new page (e.g., SettingsPage)

1. **Create the page class** (`src/classes/SettingsPage.js`):
```javascript
export class SettingsPage {
  constructor() {
    // Page state
  }

  async init() {
    this.setupElements();
    this.setupEventListeners();
  }

  setupElements() {
    // Get DOM references
  }

  setupEventListeners() {
    // Attach listeners
  }

  async beforeEnter(params) {
    // Prepare page
  }

  async onEnter() {
    // Page is now visible
  }

  async beforeLeave() {
    // Save state
  }

  hide() {
    // Cleanup
  }
}
```

2. **Register in HTML** (`src/index.html`):
```html
<!-- Add page div -->
<div id="page-settings" class="page page-settings" style="display: none;">
  <!-- Page content -->
</div>

<!-- Register in script -->
import { SettingsPage } from './classes/SettingsPage.js';
const settingsPage = new SettingsPage();
router.register('settings', settingsPage);
```

3. **Navigate to it** from another page:
```javascript
await router.navigate('settings', { param: 'value' });
```

#### Adding a new toolbar button

1. **Add button to HTML** (`src/index.html`):
```html
<button class="toolbar-btn" data-format="custom" title="Custom">
  <svg><!-- icon --></svg>
</button>
```

2. **Handle in EditorPage** (`src/classes/EditorPage.js`):
```javascript
case 'custom':
  formatted = `[custom]${selectedText}[/custom]`;
  break;
```

### Working on Backend

#### Adding a new Tauri command

1. **Add to storage** (`src-tauri/src/storage.rs`):
```rust
pub fn new_method(&self) -> Result<String, String> {
    // Implementation
    Ok("result".to_string())
}
```

2. **Create command** (`src-tauri/src/commands.rs`):
```rust
#[tauri::command]
pub fn my_command(storage: State<Mutex<Storage>>) -> ApiResponse<String> {
    match storage.lock() {
        Ok(store) => match store.new_method() {
            Ok(result) => ApiResponse::ok(result),
            Err(e) => ApiResponse::err(e),
        },
        Err(e) => ApiResponse::err(format!("Failed to lock storage: {}", e)),
    }
}
```

3. **Register in lib.rs**:
```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
])
```

4. **Add to API class** (`src/classes/API.js`):
```javascript
static async myCommand(args) {
    return this.invoke('my_command', { args });
}
```

#### Adding a new data field to Note

1. **Update model** (`src-tauri/src/models.rs`):
```rust
pub struct Note {
    // ... existing fields
    pub custom_field: String,  // Add here
}
```

2. **Update serialization** (`src-tauri/src/models.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    // Will auto-serialize with serde
}
```

3. **Update storage** (`src-tauri/src/storage.rs`):
```rust
// Update load/save methods if needed
```

## 📋 Code Standards

### JavaScript
```javascript
// Class naming: PascalCase
export class NotesPage { }

// Method naming: camelCase
async loadNotes() { }

// Private methods: _camelCase
_internalMethod() { }

// Constants: UPPER_SNAKE_CASE
const DEBOUNCE_DELAY = 500;

// Use JSDoc comments
/**
 * Brief description
 * @param {type} name - Description
 * @returns {type} Description
 */
method(name) { }
```

### Rust
```rust
// Module names: snake_case
mod models;

// Struct names: PascalCase
pub struct Note { }

// Function names: snake_case
pub fn get_notes() { }

// Constants: UPPER_SNAKE_CASE
const DEFAULT_TIMEOUT: u64 = 500;

// Add doc comments
/// Brief description
/// 
/// Longer description if needed
pub fn my_function() { }
```

### CSS
```css
/* BEM naming when needed */
.component { }
.component__element { }
.component--modifier { }

/* CSS variables for theme */
:root {
  --color-primary: #a89ff9;
  --color-bg: #0f0f0f;
}

/* Media queries at end */
@media (max-width: 768px) { }
```

## 🔧 Common Tasks

### Task: Add Search to Notes Page

1. **Backend** - Already implemented (search_notes command)

2. **Frontend** - In `NotesPage.js`:
```javascript
handleSearch(query) {
  this.searchQuery = query;
  // Filter locally or call API
  this.applyFilters();
  this.render();
}
```

3. **UI** - HTML search input already exists

### Task: Change Note Color

1. **Frontend** - `EditorPage.js`:
```javascript
async updateNoteColor(color) {
  this.currentNote.color = color;
  await this.saveNote();
}
```

2. **Backend** - Already handles in update command

### Task: Export Note to File

1. **Backend** (`commands.rs`):
```rust
#[tauri::command]
pub fn export_note(id: String, storage: State<Mutex<Storage>>) -> ApiResponse<String> {
    // Implementation
}
```

2. **Frontend** (`API.js`):
```javascript
static async exportNote(id) {
    return this.invoke('export_note', { id });
}
```

3. **Use** in `EditorPage.js`:
```javascript
const content = await API.exportNote(this.currentNote.id);
```

## 🐛 Troubleshooting

### "Note not found" error

**Cause**: Note ID doesn't exist in storage

**Solution**:
```bash
# Check storage file
cat ~/.config/nflux/notes.json  # Linux
# or
Get-Content $env:APPDATA\nflux\data\notes.json  # Windows
```

### Module not found in JavaScript

**Cause**: Incorrect import path

**Solution**: Check file path is relative and correct
```javascript
// ❌ Wrong
import { API } from './API.js';

// ✅ Right
import { API } from './classes/API.js';
```

### Rust compilation errors

**Solution**:
```bash
cd src-tauri
cargo clean
cargo fetch
cargo check
```

### Preview not updating

**Solution**: In `EditorPage.js`:
```javascript
// Make sure preview toggle is on
this.previewMode = true;

// Force update
this.updatePreview();
```

## 📚 Key Classes & Methods

### API Class
```javascript
// Static methods - no instantiation needed
API.getNotes()
API.createNote(title, content)
API.updateNote(id, request)
API.deleteNote(id)
API.searchNotes(query)
API.getNotesByStatus(archived)
API.getStats()
```

### Router Class
```javascript
router.navigate(page, params)
router.back()
router.register(name, pageInstance)
router.isActive(pageName)
router.getCurrentPage()
```

### EventEmitter
```javascript
eventBus.on('event:name', callback)
eventBus.emit('event:name', data)
eventBus.off('event:name', callback)
```

## 🚦 Development Checklist

- [ ] Run `cargo check` before commit
- [ ] Run browser console tests
- [ ] Test on different screen sizes
- [ ] Test keyboard shortcuts
- [ ] Test error cases (delete, archive, etc)
- [ ] Check localStorage data is saved
- [ ] Verify Rust command serialization
- [ ] Update CHANGELOG if needed

## 📖 References

- [Tauri Documentation](https://tauri.app)
- [MDN Web Docs](https://developer.mozilla.org)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Marked.js](https://marked.js.org/)
- [Highlight.js](https://highlightjs.org/)

## 💡 Tips

1. **Use browser DevTools**: Press F12 to debug JavaScript
2. **Check Tauri console**: Console shows both JS and Rust messages
3. **Reload in dev**: Changes reflect immediately
4. **Check file storage**: Notes saved to JSON file, not localStorage
5. **Use event bus**: Components communicate via events, not direct calls

---

**Happy coding! Feel free to extend this guide as you add more features.**
