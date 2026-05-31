# NFlux - Quick Reference

## 🚀 Quick Start
```bash
npm run tauri dev      # Start development
npm run tauri build    # Build production
cd src-tauri && cargo check  # Verify Rust
```

## 📁 File Quick Reference

| File | Purpose | Size |
|------|---------|------|
| `src/index.html` | Two-page layout (notes + editor) | 200+ lines |
| `src/styles.css` | Complete dark theme | 900+ lines |
| `src/classes/API.js` | Tauri command handler | 50 lines |
| `src/classes/Router.js` | Page navigation | 100 lines |
| `src/classes/EventEmitter.js` | Event system | 80 lines |
| `src/classes/NotesPage.js` | Notes selector | 400 lines |
| `src/classes/EditorPage.js` | Markdown editor | 500 lines |
| `src-tauri/src/models.rs` | Data structures | 200 lines |
| `src-tauri/src/storage.rs` | File persistence | 250 lines |
| `src-tauri/src/commands.rs` | Tauri handlers | 100 lines |

## 🎯 Key Concepts

### Two-Page Architecture
```
NotesPage (selector)  ←→  EditorPage (editor)
    ↓                           ↓
Find/create note         Edit markdown
Apply filters           Format text
Search notes            Live preview
```

### Data Flow
```
UI (HTML/CSS) → JavaScript Classes → Tauri API ↔ Rust Backend
                (API/Router)
```

### Class Hierarchy
```
EventEmitter (global event bus)
    ↓
Router (page manager)
    ├── NotesPage
    └── EditorPage

API (static Tauri handler)
```

## 🔧 Common Commands

### Development
```bash
npm run tauri dev        # Dev server with hot reload
npm run tauri build      # Production build
npm install              # Install deps
```

### Rust Testing
```bash
cd src-tauri
cargo check             # Quick check
cargo build             # Full build
cargo clean             # Clean build
cargo update            # Update deps
```

### File Locations
```bash
# Windows
%APPDATA%\nflux\data\notes.json

# macOS
~/Library/Application Support/nflux/notes.json

# Linux
~/.config/nflux/notes.json
```

## 📝 Quick Tasks

### Add a new toolbar button
1. Add `<button>` with `data-format` in HTML
2. Add case in `EditorPage.applyFormat()`

### Add a new page
1. Create class extending from base pattern
2. Register: `router.register(name, page)`
3. Navigate: `router.navigate('name')`

### Add a new command
1. Add method to `Storage` (Rust)
2. Create `#[tauri::command]` (Rust)
3. Register in `generate_handler![]` (Rust)
4. Add method to `API` class (JS)

### Handle errors
```javascript
try {
  const result = await API.command();
  // Use result
} catch (error) {
  console.error('Failed:', error);
  this.showError('User-friendly message');
}
```

## 🎨 Color Palette

| Color | Hex | Usage |
|-------|-----|-------|
| Primary | `#a89ff9` | Accent, active state |
| Background | `#0f0f0f` | Main background |
| Surface | `#161616` | Cards, panels |
| Border | `#2a2a2a` | Dividers, borders |
| Text | `#e8e8e8` | Primary text |
| Muted | `#888` | Secondary text |
| Note Red | `#f28482` | Note color |
| Note Green | `#81c995` | Note color |
| Note Blue | `#81c7f5` | Note color |
| Note Purple | `#c5a3ff` | Note color |

## ⌨️ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Ctrl+N` | New note |
| `Ctrl+S` | Save note |
| `Ctrl+B` | Bold |
| `Ctrl+I` | Italic |
| `Ctrl+K` | Link |
| `Escape` | Go back |
| `↑/↓` | Navigate notes |

## 🔍 Debugging Tips

### JavaScript Console (F12)
```javascript
// Available globals in dev
window.API           // Tauri commands
window.router        // Page router
window.eventBus      // Event system
window.NotesPage     // Page instance
window.EditorPage    // Page instance

// Examples
router.navigate('notes')
eventBus.emit('note:updated', note)
API.getNotes()
```

### Rust Logging
```bash
# Enable logging
RUST_LOG=debug npm run tauri dev

# In code
println!("Debug: {:?}", variable);
eprintln!("Error: {:?}", error);
```

### Check Storage
```bash
# Windows PowerShell
Get-Content $env:APPDATA\nflux\data\notes.json | ConvertFrom-Json

# macOS/Linux
cat ~/.config/nflux/notes.json | jq
```

## 📊 Architecture Decisions

| Choice | Reason |
|--------|--------|
| Two separate pages | Better UX, clear separation |
| Class-based JS | Scalability, organization |
| Rust backend | Performance, type safety |
| JSON storage | Simple, portable, debuggable |
| EventEmitter | Loose coupling, extensible |
| Markdown parser | Industry standard |
| Tauri | Lightweight, secure |

## ✅ Checklist for New Features

- [ ] Add Rust command if needed
- [ ] Add JS method to API class
- [ ] Update UI HTML/CSS
- [ ] Add to appropriate Page class
- [ ] Handle errors gracefully
- [ ] Update documentation
- [ ] Test on desktop
- [ ] Check responsive design
- [ ] Verify keyboard shortcuts work
- [ ] Test error cases

## 🐛 Common Issues

| Problem | Solution |
|---------|----------|
| Notes not loading | Check notes.json exists & is valid |
| Preview not showing | Click preview toggle, ensure mode is on |
| Keyboard shortcut not working | Check if target is editor element |
| Rust won't compile | Run `cargo clean && cargo fetch` |
| Tauri commands fail | Check error in browser console |
| Styles not applying | Clear browser cache, reload |

## 📚 Resource Links

- [Tauri Docs](https://tauri.app/develop/getting-started/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [MDN Web Docs](https://developer.mozilla.org/)
- [CommonMark Spec](https://spec.commonmark.org/)
- [Marked.js Docs](https://marked.js.org/)

## 📞 Support

For questions or issues:
1. Check DEVELOPMENT.md for detailed guide
2. Check ARCHITECTURE.md for design patterns
3. Review code comments
4. Check browser console for errors
5. Check Tauri console for backend errors

---

**Last Updated**: 2024
**Version**: 2.0 (Class-Based Architecture)
