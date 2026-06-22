// ASHOT Creator Studio 桌面壳:加载 creator.ashot.live。
// 关键:给 webview 设真实桌面 Chrome 的 User-Agent,绕过 Google OAuth
// 对嵌入式 webview 的拦截(disallowed_useragent),让 Google 登录可用。

const WIN_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";
const MAC_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let ua = if cfg!(target_os = "windows") { WIN_UA } else { MAC_UA };
            let url = tauri::WebviewUrl::External(
                "https://creator.ashot.live/".parse().expect("valid url"),
            );
            tauri::WebviewWindowBuilder::new(app, "main", url)
                .title("ASHOT Creator Studio")
                .inner_size(1280.0, 832.0)
                .min_inner_size(960.0, 600.0)
                .resizable(true)
                .user_agent(ua)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
