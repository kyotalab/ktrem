use crate::error::KtermError;
use crate::model::config::Config;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn load_or_setup() -> Result<Config, KtermError> {
    let path = config_path();
    if path.exists() {
        load()
    } else {
        setup()
    }
}

fn load() -> Result<Config, KtermError> {
    let path = config_path();
    let content =
        fs::read_to_string(&path).map_err(|e| KtermError::ConfigReadError(e.to_string()))?;
    let config: Config =
        toml::from_str(&content).map_err(|e| KtermError::ConfigParseError(e.to_string()))?;
    Ok(config)
}

fn setup() -> Result<Config, KtermError> {
    let candidates = find_workspace_candidates();

    let workspace = match candidates.len() {
        0 => {
            // 候補なし → 手動入力
            print!("Kasten workspace not found. Please enter the path: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| KtermError::SetupError(e.to_string()))?;
            PathBuf::from(input.trim())
        }
        1 => {
            // 候補1件 → 確認
            print!("Found: {:?}\nUse this? [Y/n]: ", candidates[0]);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| KtermError::SetupError(e.to_string()))?;
            if input.trim().to_lowercase() == "n" {
                print!("Please enter the path: ");
                io::stdout().flush().unwrap();
                let mut path_input = String::new();
                io::stdin()
                    .read_line(&mut path_input)
                    .map_err(|e| KtermError::SetupError(e.to_string()))?;
                PathBuf::from(path_input.trim())
            } else {
                candidates[0].clone()
            }
        }
        _ => {
            // 候補複数 → 一覧表示して選択
            println!("Multiple workspaces found:");
            for (i, candidate) in candidates.iter().enumerate() {
                println!("  {}: {:?}", i + 1, candidate);
            }
            print!("Select [1-{}]: ", candidates.len());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| KtermError::SetupError(e.to_string()))?;
            let selected: usize = input
                .trim()
                .parse()
                .map_err(|_| KtermError::SetupError("Invalid selection".to_string()))?;
            if selected < 1 || selected > candidates.len() {
                return Err(KtermError::SetupError("Out of range".to_string()));
            }
            candidates[selected - 1].clone()
        }
    };

    let config = Config { workspace };
    save(&config)?;
    Ok(config)
}

fn find_workspace_candidates() -> Vec<PathBuf> {
    let mut search_dirs = Vec::new();

    // ~/Documents もスキャン対象に追加
    if let Some(home) = dirs::home_dir() {
        search_dirs.push(home.join("Documents"));
        search_dirs.push(home.join("Library").join("Mobile Documents"));
    }

    let mut candidates = Vec::new();

    for base in search_dirs {
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                let path = entry.path();

                // 隠しディレクトリをスキップ
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with('.'))
                    .unwrap_or(false)
                {
                    continue;
                }

                if path.join("cards").is_dir() && path.join("index.json").exists() {
                    candidates.push(path);
                }
            }
        }
    }

    candidates
}

fn save(config: &Config) -> Result<(), KtermError> {
    let path = config_path();
    // ディレクトリがなければ作成
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| KtermError::ConfigWriteError(e.to_string()))?;
    }
    let content =
        toml::to_string(config).map_err(|e| KtermError::ConfigParseError(e.to_string()))?;
    fs::write(&path, content).map_err(|e| KtermError::ConfigWriteError(e.to_string()))?;
    Ok(())
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".config")
        .join("kterm")
        .join("config.toml")
}
