## 1.0.0
### Breaking Changes
- Changed `--browser` flag to `--no-browser` - browser now opens automatically by default (previously required `--browser` flag to open)
- Changed `-s/--highlight` flag to `--no-highlight` - syntax highlighting now enabled by default (previously required `-s` flag to enable)
- Changed `-k/--katex` flag to `--no-katex` - KaTeX math rendering now enabled by default (previously required `-k` flag to enable)
- Changed `-d/--d2` flag to `--no-d2` - D2 diagram rendering now enabled by default (previously required `-d` flag to enable)

### Dependency Upgrades
- Upgraded tokio from 0.2 to 1.40
- Upgraded hyper from 0.13 to 0.14
- Upgraded hyper-staticfile from 0.5 to 0.9.6
- Removed deprecated tokio-fs, tokio-io, and tokio-sync dependencies (now using tokio 1.x built-in equivalents)

### Features & Improvements
- Browser now opens automatically by default when starting the server
- Syntax highlighting (Prism.js) now enabled by default
- KaTeX math rendering now enabled by default
- D2 diagram rendering now enabled by default (inline SVG rendering from D2 code blocks)
- Added Mermaid.js diagram support (always enabled, activates automatically for mermaid code blocks)
- Automatic port detection: tries ports 10009-10018 to enable running multiple instances simultaneously
- Static files now served relative to markdown file's directory (fixes inline image references)
- Use `--no-browser` flag to disable automatic browser opening
- Use `--no-highlight` flag to disable syntax highlighting
- Use `--no-katex` flag to disable KaTeX math rendering
- Use `--no-d2` flag to disable D2 diagram rendering
- Better error handling for browser opening with user-friendly error messages
- Improved D2 warning message to indicate it's enabled by default
- Notification displayed when using non-default port due to port conflict
- Graceful failure message when all 10 ports are occupied
- Added comprehensive documentation to openurl.rs module
- Support for more Unix-like systems (BSD, etc.) in browser opening
- Improved console output with clearer status messages

### Bug Fixes & Code Quality
- Extracted template strings to external files using `include_str!()` (reduced template.rs from 3,912 to 90 lines)
- Moved HTML/CSS content to dedicated template files in `templates/` directory for better maintainability
- Removed unused `static_file()` function (dead code)
- Removed unused UrlOpen trait and implementation
- Removed unused mermaid module from lib.rs
- Fixed compiler warnings for unused imports (Error, info, Watcher)
- Fixed compiler warnings for unnecessary `mut` declarations
- Updated to newer notify API with proper error handling
- Fixed deprecated hyper-staticfile API usage (ResolveResult enum variants)
- Added `#[allow(dead_code)]` annotation for `format_boilerplate_no_preview()`

## 0.1.1
- add lib
- async refactor
- auto refresh 

## 0.1.2
- add static file serving with hyper-staticfile
- add katex support
- now on cargo
- include license for cargo
