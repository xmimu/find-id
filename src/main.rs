// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashSet, path::PathBuf, thread};

use find_id_ui::{Config, MatchInfo, find_id};
use rfd::FileDialog;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn load_config(ui: &MainWindow, config_file: &str) {
    let mut config: Config = Config::new();

    if let Ok(c) = Config::load(config_file) {
        config = c;
    }
    ui.set_path(SharedString::from(&config.path));
    ui.set_check_guid(config.check_guid);
    ui.set_check_short_id(config.check_short_id);
    ui.set_check_media_id(config.check_media_id);
}

fn save_config(ui: &MainWindow, config_file: &str) -> Result<(), std::io::Error> {
    let mut config: Config = Config::new();
    config.path = ui.get_path().to_string();
    config.check_guid = ui.get_check_guid();
    config.check_short_id = ui.get_check_short_id();
    config.check_media_id = ui.get_check_media_id();
    config.save(config_file)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = "config.json";
    let ui = MainWindow::new()?;
    let ui_weak = ui.as_weak();

    load_config(&ui, config_file);

    ui.on_run_browser_path({
        let ui_weak = ui_weak.clone();
        move || {
            let path = FileDialog::new()
                .add_filter("wproj", &["wproj"])
                .pick_file();
            if let Some(path) = path {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_path(SharedString::from(path.to_string_lossy().to_string()));
                }
            }
        }
    });

    ui.on_run_query({
        let ui_weak = ui_weak.clone();
        move |path, query, check_guid, check_short_id, check_media_id| {
            let ui_weak = ui_weak.clone();
            save_config(&ui_weak.unwrap(), config_file).unwrap();

            thread::spawn(move || {
                // 查询处理
                let p = PathBuf::from(&path.to_string())
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let result = find_id(
                    &query.to_string(),
                    &p,
                    check_guid,
                    check_short_id,
                    check_media_id,
                );

                // 去除重复项
                let set: HashSet<MatchInfo> = result.into_iter().collect();
                let data: Vec<SharedString> = set
                    .into_iter()
                    .map(|x| SharedString::from(x.to_string()))
                    .collect();

                // 更新 UI
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let model = ModelRc::new(VecModel::from(data));
                        ui.set_table_data(model);
                    }
                })
                .unwrap();
            });
        }
    });

    ui.run()?;

    Ok(())
}
