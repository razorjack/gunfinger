//! The supported platforms, macOS and Linux (ADR 0020). Where the harness
//! runs a program that only one of them has, one function per need takes
//! a `Platform` and returns the command, so that every branch is tested
//! on either system.

/// A system whose own programs the harness runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOs,
    /// Linux, and any other system: freedesktop.org's `xdg-open`, eSpeak NG.
    Linux,
}

impl Platform {
    /// The system this build runs on.
    pub const CURRENT: Platform = if cfg!(target_os = "macos") {
        Platform::MacOs
    } else {
        Platform::Linux
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[test]
    fn a_macos_build_runs_the_programs_of_macos() {
        assert_eq!(Platform::CURRENT, Platform::MacOs);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_linux_build_runs_the_programs_of_linux() {
        assert_eq!(Platform::CURRENT, Platform::Linux);
    }
}
