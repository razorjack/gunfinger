//! The desktop's programs the page asks for: one opens the page in the
//! default browser, the other shows a file in the file manager. macOS has
//! `open`; Linux has freedesktop.org's `xdg-open` (ADR 0020).

use std::path::Path;
use std::process::Command;

use crate::platform::Platform;

/// The command that opens `url` in the default browser.
pub fn browser(platform: Platform, url: &str) -> Command {
    let program = match platform {
        Platform::MacOs => "open",
        Platform::Linux => "xdg-open",
    };
    let mut command = Command::new(program);
    command.arg(url);
    command
}

/// The command that shows `file` in the file manager. The Finder selects
/// it (`open -R`). `xdg-open` opens its folder: selecting a file there
/// needs a D-Bus call that not every file manager answers.
pub fn reveal(platform: Platform, file: &Path) -> Command {
    match platform {
        Platform::MacOs => {
            let mut command = Command::new("open");
            command.arg("-R").arg(file);
            command
        }
        Platform::Linux => {
            let mut command = Command::new("xdg-open");
            command.arg(file.parent().unwrap_or(file));
            command
        }
    }
}

/// The program and its arguments joined by spaces, for messages that
/// name what ran.
pub fn shown(command: &Command) -> String {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(|part| part.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(command: &Command) -> Vec<String> {
        std::iter::once(command.get_program())
            .chain(command.get_args())
            .map(|part| part.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn macos_opens_the_page_and_selects_the_file_with_open() {
        let url = "http://127.0.0.1:8123/token/";
        let file = Path::new("/music/label/Artist - Title.opus");

        assert_eq!(parts(&browser(Platform::MacOs, url)), ["open", url]);
        assert_eq!(
            parts(&reveal(Platform::MacOs, file)),
            ["open", "-R", "/music/label/Artist - Title.opus"]
        );
    }

    #[test]
    fn linux_opens_the_page_and_the_files_folder_with_xdg_open() {
        let url = "http://127.0.0.1:8123/token/";
        let file = Path::new("/music/label/Artist - Title.opus");

        assert_eq!(parts(&browser(Platform::Linux, url)), ["xdg-open", url]);
        assert_eq!(
            parts(&reveal(Platform::Linux, file)),
            ["xdg-open", "/music/label"]
        );
    }

    #[test]
    fn a_command_is_shown_as_its_program_and_arguments() {
        let file = Path::new("/music/label/a.mp3");

        assert_eq!(
            shown(&reveal(Platform::MacOs, file)),
            "open -R /music/label/a.mp3"
        );
        assert_eq!(
            shown(&reveal(Platform::Linux, file)),
            "xdg-open /music/label"
        );
    }
}
