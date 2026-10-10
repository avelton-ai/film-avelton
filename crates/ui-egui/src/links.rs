//! Community and project links (Help menu, About dialog, header Discord button, Home screen).
//!
//! Film Avelton is part of the Avelton family: the community lives on the Avelton Discord, every
//! app has a page on the Avelton website and a public GitHub repository.

/// The app's short name, used in the website and repository URLs.
pub const APP: &str = "filmcraft";

/// The Avelton community Discord.
pub const DISCORD: &str = "https://github.com/avelton-ai/film-avelton/issues";
/// The Avelton website.
pub const WEBSITE: &str = "https://avelton.online";
/// Film Avelton's page on the Avelton website.
pub const APP_PAGE: &str = "https://film.avelton.online";
/// Film Avelton's source repository.
pub const GITHUB: &str = "https://github.com/avelton-ai/film-avelton";
/// New issue on Film Avelton's repository.
pub const ISSUES: &str = "https://github.com/avelton-ai/film-avelton/issues";

/// (command id, label, url) for every link, in menu order.
pub const ALL: [(&str, &str, &str); 5] = [
    ("help.discord", "Avelton support…", DISCORD),
    ("help.website", "Avelton Website", WEBSITE),
    ("help.appPage", "Film Avelton on avelton.online", APP_PAGE),
    ("help.github", "Film Avelton on GitHub", GITHUB),
    ("help.reportIssue", "Report an Issue…", ISSUES),
];

/// The URL a `help.*` link command opens.
pub fn url_for(command: &str) -> Option<&'static str> {
    ALL.iter().find(|(id, _, _)| *id == command).map(|(_, _, u)| *u)
}

/// Open `url` in the system browser (a new tab on the web).
pub fn open(ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_point_to_avelton() {
        assert_eq!(APP_PAGE, "https://film.avelton.online");
        assert_eq!(GITHUB, "https://github.com/avelton-ai/film-avelton");
        assert!(ALL.iter().all(|(id, _, u)| id.starts_with("help.") && u.starts_with("https://")));
        assert_eq!(url_for("help.discord"), Some(DISCORD));
        assert_eq!(url_for("help.nope"), None);
    }
}
