use tauri::{Manager, WebviewWindow};
use tauri::Emitter;
use warp::Filter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            tokio::spawn(run_server(window.clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn run_server(window: WebviewWindow) {
    let notify = warp::path("notification")
        .and(warp::post())
        .and(warp::body::json::<String>())
        .map(move |msg: String| {
            let w = window.clone();
            tauri::async_runtime::spawn_blocking(move || {
               let _ = w.app_handle().emit("new-notification", msg.clone());
            });
            warp::reply::json(&"ok")
            
        });

    warp::serve(notify).run(([127, 0, 0, 1], 8080)).await;
}