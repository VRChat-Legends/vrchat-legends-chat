// VRChat Legends Chat: the site's Messages in its own window.
//
// The window opens https://vrchatlegends.com/messages. The site sees "VRCLDesktop" in the browser
// name (and window.__VRCL_DESKTOP__) and shows just the chat (src/components/layout/DesktopApp.jsx).
// Signing in uses the site's normal sign-in, so people log in with their VRChat Legends account.
// Anything that leaves the chat or sign-in opens in the normal web browser instead.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Url, WebviewUrl, WebviewWindowBuilder};

const START_URL: &str = "https://vrchatlegends.com/messages";

/// Check GitHub for a newer release, install it, and offer to restart. Signed updates only
/// (see the pubkey in tauri.conf.json), so nothing unsigned can be installed. Release builds only.
#[cfg(not(debug_assertions))]
async fn check_for_updates(app: tauri::AppHandle) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    use tauri_plugin_updater::UpdaterExt;

    let update = match app.updater() {
        Ok(updater) => match updater.check().await {
            Ok(Some(update)) => update,
            _ => return, // up to date, or could not reach GitHub
        },
        Err(_) => return,
    };

    let version = update.version.clone();
    if update.download_and_install(|_received, _total| {}, || {}).await.is_err() {
        return;
    }

    let restart = app
        .dialog()
        .message(format!(
            "Version {version} has been installed. Restart VRChat Legends Chat now to finish updating?"
        ))
        .title("Update ready")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Restart now".into(),
            "Later".into(),
        ))
        .blocking_show();
    if restart {
        app.restart();
    }
}

#[cfg(target_os = "windows")]
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0 VRCLDesktop/0.1";
#[cfg(not(target_os = "windows"))]
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15 VRCLDesktop/0.1";

/// Site pages that stay in the app window: the chat and everything signing in needs.
const SITE_PATHS: &[&str] = &[
    "/messages", "/auth", "/login", "/oauth", "/signin", "/sign-in", "/register",
    "/account/verify", "/account/security", "/_next", "/api",
];

fn under(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{prefix}/"))
}

/// Does this address stay in the app window? Everything else opens in the web browser.
fn stays_in_app(url: &Url) -> bool {
    match url.scheme() {
        "https" => {}
        "about" | "data" | "blob" => return true,
        _ => return url.host_str() == Some("tauri.localhost"),
    }
    let host = url.host_str().unwrap_or("");
    let path = url.path();
    match host {
        "vrchatlegends.com" | "www.vrchatlegends.com" => path == "/" || SITE_PATHS.iter().any(|p| under(path, p)),
        "api.vrchatlegends.com" => true,
        // sign-in with Discord, Google, GitHub or Twitch
        "discord.com" | "www.discord.com" => under(path, "/oauth2") || under(path, "/login") || under(path, "/api/oauth2"),
        "accounts.google.com" | "accounts.youtube.com" => true,
        "github.com" => under(path, "/login") || under(path, "/session") || under(path, "/sessions"),
        "id.twitch.tv" => true,
        "www.twitch.tv" | "twitch.tv" => under(path, "/login") || under(path, "/passport-callback"),
        // video players inside link previews
        "www.youtube-nocookie.com" => under(path, "/embed"),
        _ => false,
    }
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn main() {
    tauri::Builder::default()
        // opening the app again brings the open window forward
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_main(app)))
        // remembers the window size and place
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // look for a new version in the background, once, on launch
            #[cfg(not(debug_assertions))]
            tauri::async_runtime::spawn(check_for_updates(app.handle().clone()));
            let start = Url::parse(START_URL).expect("start address");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start))
                .title("VRChat Legends Chat")
                .inner_size(1280.0, 820.0)
                .min_inner_size(420.0, 560.0)
                .user_agent(USER_AGENT)
                .initialization_script("window.__VRCL_DESKTOP__ = true;")
                .on_navigation(|url| {
                    if stays_in_app(url) {
                        return true;
                    }
                    let _ = open::that_detached(url.as_str());
                    false
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running VRChat Legends Chat");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stays(u: &str) -> bool {
        stays_in_app(&Url::parse(u).unwrap())
    }

    #[test]
    fn chat_and_sign_in_stay_other_pages_open_in_the_browser() {
        assert!(stays("https://vrchatlegends.com/messages?with=abc"));
        assert!(stays("https://vrchatlegends.com/auth/login?returnTo=%2Fmessages"));
        assert!(stays("https://api.vrchatlegends.com/api/oauth/discord"));
        assert!(stays("https://discord.com/oauth2/authorize?client_id=1"));
        assert!(stays("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ"));
        assert!(!stays("https://vrchatlegends.com/legends"));
        assert!(!stays("https://discord.com/channels/1/2"));
        assert!(!stays("https://example.com/"));
        assert!(!stays("http://vrchatlegends.com/messages"));
    }
}
