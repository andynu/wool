//! Cross-platform browser URL opening functionality.
//!
//! This module provides utilities to open URLs in the system's default web browser
//! across different operating systems. It uses platform-specific commands:
//!
//! - **Linux**: `xdg-open`
//! - **macOS**: `open`
//! - **Windows**: `cmd /C start`
//! - **Other Unix-like systems**: `xdg-open` with error handling
//!
//! # Example
//!
//! ```no_run
//! use url::Url;
//! use wool::openurl;
//!
//! let url = Url::parse("http://localhost:10009").unwrap();
//! openurl::open(&url);
//! ```
//!
//! # Attribution
//!
//! Original concept from: <https://github.com/overdrivenpotato/url_open>

use url::Url;

/// Trait providing convenient URL opening functionality.
///
/// This trait extends the `Url` type with an `open()` method that
/// launches the URL in the system's default browser.
pub trait UrlOpen {
    /// Opens this URL in the system's default browser.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use url::Url;
    /// use wool::openurl::UrlOpen;
    ///
    /// let url = Url::parse("http://example.com").unwrap();
    /// url.open();
    /// ```
    fn open(&self);
}

impl UrlOpen for Url {
    fn open(&self) {
        open(self);
    }
}

/// Opens a URL in the default browser on Windows.
///
/// Uses the Windows command `cmd /C start` to launch the URL. The empty string
/// argument after "start" is the window title parameter, which prevents URLs
/// starting with quotes from being misinterpreted.
///
/// This function is only compiled on Windows targets.
#[cfg(target_os = "windows")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("cmd")
        .args(&["/C", "start", "", url.as_str()])
        .output();
}

/// Opens a URL in the default browser on macOS.
///
/// Uses the macOS `open` command to launch the URL in the default browser.
///
/// This function is only compiled on macOS targets.
#[cfg(target_os = "macos")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("open")
        .arg(url.as_str())
        .output();
}

/// Opens a URL in the default browser on Linux.
///
/// Uses the freedesktop.org standard `xdg-open` utility, which is the
/// standard way to open URLs on Linux desktop environments.
///
/// This function is only compiled on Linux targets.
#[cfg(target_os = "linux")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("xdg-open")
        .arg(url.as_str())
        .output();
}

/// Opens a URL in the default browser on other Unix-like systems.
///
/// Attempts to use `xdg-open` as it's commonly available on BSD and other
/// Unix-like systems. If the command fails, prints an error message with
/// the URL so the user can manually navigate to it.
///
/// This function is compiled on Unix-like systems other than Linux and macOS,
/// such as FreeBSD, OpenBSD, NetBSD, etc.
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn open(url: &Url) {
    if std::process::Command::new("xdg-open")
        .arg(url.as_str())
        .output()
        .is_err()
    {
        eprintln!("Failed to open browser. Please navigate to: {}", url);
    }
}
