# NFlux - Markdown Note Editor

A lightweight, fast markdown note editor built with **Tauri** and **Vanilla JavaScript**. Inspired by Google's Android Notes app with a modern, dark UI.

## ✨ Features

### Core Features
- **📝 Markdown Editing** - Full CommonMark support with real-time preview
- **🎨 Syntax Highlighting** - Beautiful code blocks with Highlight.js
- **👁️ Live Preview** - Split-screen preview panel for instant markdown rendering
- **🏷️ Color Tags** - 8 color options to organize and quickly identify notes
- **📦 Archive System** - Hide completed or old notes without deleting them
- **🔍 Search** - Full-text search across all notes (title and content)

### Formatting Tools
- **Text Formatting**: Bold, Italic, Strikethrough
- **Headings**: H1, H2, H3
- **Lists**: Bullet points and numbered lists
- **Code**: Inline code and code blocks with syntax highlighting
- **Quotes**: Blockquotes
- **Links**: Markdown link support
- **Meta Info**: Word count and character count display

### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Ctrl+N` / `Cmd+N` | New note |
| `Ctrl+B` / `Cmd+B` | Bold text |
| `Ctrl+I` / `Cmd+I` | Italic text |
| `Ctrl+K` / `Cmd+K` | Insert link |
| `Ctrl+.` / `Cmd+.` | Bullet list |

### UI/UX
- **Split Layout** - Sidebar with notes list, main editor area
- **Responsive Design** - Works on desktop and tablet sizes
- **Dark Theme** - Eye-friendly dark mode by default
- **Auto-Save** - Notes automatically save as you type (500ms debounce)
- **Local Storage** - All notes stored securely in browser localStorage
- **Persistent State** - Notes persist across sessions

## 🚀 Getting Started

### Requirements
- Node.js (for dependencies)
- Tauri CLI
- Rust (for Tauri backend)

### Installation

```bash
# Install dependencies
npm install

# Development
npm run tauri dev

# Build
npm run tauri build
```

## 📁 Project Structure

```
src/
├── index.html      # Main HTML structure
├── main.js         # App logic and state management
├── styles.css      # Styling and animations
└── assets/         # Icons and images
```

## 🎨 Technology Stack

- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **Markdown Parsing**: [marked](https://marked.js.org/)
- **Syntax Highlighting**: [Highlight.js](https://highlightjs.org/)
- **Desktop Framework**: [Tauri](https://tauri.app/)
- **Storage**: Browser localStorage

## 📊 Note Object Structure

```javascript
{
  id: string,              // Unique identifier
  title: string,           // Note title
  content: string,         // Markdown content
  color: string,           // 'default' | 'red' | 'orange' | 'yellow' | 'green' | 'teal' | 'blue' | 'purple'
  archived: boolean,       // Archive status
  createdAt: ISO string,   // Creation timestamp
  updatedAt: ISO string    // Last update timestamp
}
```

## 🎯 Roadmap

- [ ] Export notes to PDF
- [ ] Export to markdown files
- [ ] Tags/Labels system
- [ ] Collaborative editing
- [ ] Cloud sync
- [ ] Mobile app
- [ ] Custom themes
- [ ] Spell checker
- [ ] Note templates

## 📝 License

See LICENSE file

## 💬 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
