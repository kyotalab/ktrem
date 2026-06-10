use crate::error::KtermError;
use crate::model::config::Config;

// config.tomlを読み込む、なければセットアップを実行
pub fn load_or_setup() -> Result<Config, KtermError>;

// config.tomlからConfigを読み込む
fn load() -> Result<Config, KtermError>;

// 初回セットアップ（ワークスペースを探して保存）
fn setup() -> Result<Config, KtermError>;

// iCloud Drive以下をスキャンしてkastenらしいディレクトリを探す
fn find_workspace_candidates() -> Vec<PathBuf>;

// config.tomlに保存
fn save(config: &Config) -> Result<(), KtermError>;
