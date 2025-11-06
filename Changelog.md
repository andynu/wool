## 1.0.0
### Breaking Changes
- Changed `--browser` flag to `--no-browser` - browser now opens automatically by default (previously required `--browser` flag to open)

### Dependency Upgrades
- Upgraded tokio from 0.2 to 1.40
- Upgraded hyper from 0.13 to 0.14
- Upgraded hyper-staticfile from 0.5 to 0.9.6
- Removed deprecated tokio-fs, tokio-io, and tokio-sync dependencies (now using tokio 1.x built-in equivalents)

### Features & Improvements
- Browser now opens automatically by default when starting the server
- Use `--no-browser` flag to disable automatic browser opening
- Better error handling for browser opening with user-friendly error messages
- Added comprehensive documentation to openurl.rs module
- Support for more Unix-like systems (BSD, etc.) in browser opening
- Improved console output with clearer status messages

### Bug Fixes & Code Quality
- Removed unused `static_file()` function (dead code)
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
