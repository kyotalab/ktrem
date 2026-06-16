use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use kterm::app::{App, AppMode, Tab, WizardField};
use kterm::ui::layout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Appの初期化
    let mut app = App::new()?;

    // ターミナルセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // メインループ
    let result = run(&mut terminal, &mut app);

    // ターミナル後片付け
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut ratatui::Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // 描画
        terminal.draw(|frame| layout::render(frame, app))?;

        // キー入力処理
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            let prev_mode = app.mode.clone(); // Cloneが必要

            match app.mode {
                AppMode::Normal => handle_normal(app, key.code, terminal)?,
                AppMode::Search => handle_search(app, key.code),
                AppMode::Wizard => handle_wizard(app, key.code)?,
                AppMode::TagEdit => handle_tag_edit(app, key.code)?,
                AppMode::ConfirmDelete => handle_confirm_delete(app, key.code)?,
                AppMode::WorkspaceSwitch => handle_workspace_switch(app, key.code)?,
                AppMode::Help => handle_help(app, key.code),
            }

            // 終了（処理前にNormalモードだった場合のみ）
            if matches!(prev_mode, AppMode::Normal)
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break;
            }
        }
    }

    Ok(())
}

fn handle_normal(
    app: &mut App,
    key: KeyCode,
    terminal: &mut ratatui::Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('j') | KeyCode::Down => app.move_down(),
        KeyCode::Char('k') | KeyCode::Up => app.move_up(),
        KeyCode::Tab => app.toggle_tab(),
        KeyCode::Char('l') | KeyCode::Right => {
            if matches!(app.tab, Tab::Zettelkasten) {
                app.toggle_expand();
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if matches!(app.tab, Tab::Zettelkasten) {
                app.toggle_expand();
            }
        }
        KeyCode::Char('n') => {
            match app.tab {
                Tab::Scratch => {
                    // Scratchを即作成してエディタで開く
                    let scratch = kterm::store::scratch::create(&app.config.scratch_dir())?;
                    let path = app
                        .config
                        .scratch_dir()
                        .join(format!("{}.md", scratch.timestamp));
                    app.scratches.insert(0, scratch);
                    open_editor(terminal, &path)?;
                    app.reload()?;
                }
                Tab::Zettelkasten => {
                    app.open_wizard();
                }
            }
        }
        KeyCode::Char('p') => {
            if matches!(app.tab, Tab::Scratch) {
                app.open_wizard();
            }
        }
        KeyCode::Char('/') => {
            app.mode = AppMode::Search;
            app.search_query = Some(String::new());
        }
        KeyCode::Enter => {
            // エディタで開く
            let path = match app.tab {
                Tab::Scratch => app
                    .scratches
                    .get(app.selected_index)
                    .map(|s| app.config.scratch_dir().join(format!("{}.md", s.timestamp))),
                Tab::Zettelkasten => app
                    .zettels
                    .get(app.selected_index)
                    .map(|z| app.config.cards_dir().join(format!("{}.md", z.file_name))),
            };
            if let Some(p) = path {
                open_editor(terminal, &p)?;
                app.reload()?;
            }
        }
        KeyCode::Char('t') => {
            if matches!(app.tab, Tab::Zettelkasten) {
                app.open_tag_edit();
            }
        }
        KeyCode::Char('d') => {
            // 削除確認は今後実装
            app.open_confirm_delete();
        }
        KeyCode::Char('w') => {
            app.open_workspace_switch();
        }
        KeyCode::Char('?') => {
            app.open_help();
        }
        _ => {}
    }
    Ok(())
}

fn handle_search(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            app.search_query = None;
        }
        KeyCode::Enter => {
            app.mode = AppMode::Normal;
        }
        KeyCode::Char(c) => {
            if let Some(query) = &mut app.search_query {
                query.push(c);
                let query = query.clone();
                app.update_search(query);
            }
        }
        KeyCode::Backspace => {
            if let Some(query) = &mut app.search_query {
                query.pop();
                let query = query.clone();
                app.update_search(query);
            }
        }
        _ => {}
    }
}

fn handle_wizard(app: &mut App, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => app.close_wizard(),
        KeyCode::Tab => {
            // フォーカス移動
            if let Some(state) = &mut app.wizard_state {
                state.focused_field = match state.focused_field {
                    WizardField::Id => WizardField::Title,
                    WizardField::Title => WizardField::Tags,
                    WizardField::Tags => WizardField::Id,
                };
            }
        }
        KeyCode::Enter => {
            // 昇格実行
            if let Some(state) = &app.wizard_state {
                let id = state.id.clone();
                let title = state.title.clone();
                let tags: Vec<String> = state
                    .tags
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();

                // ファイル名に変換（"1/1" → "1-1"）
                let file_name = id.replace('/', "-");

                let content = format!("# {}\n\n", title);
                let zettel = kterm::model::note::Zettel {
                    id: id.clone(),
                    file_name: file_name.clone(),
                    tags: tags.clone(),
                    created: chrono::Utc::now(),
                    updated: chrono::Utc::now(),
                    content,
                };

                // ファイル作成
                kterm::store::zettelkasten::create(&app.config.cards_dir(), &zettel)?;

                // index.json更新
                let entry = kterm::model::index::IndexEntry {
                    status: kterm::model::index::CardStatus::Permanent,
                    tags,
                    created: chrono::Utc::now(),
                    updated: chrono::Utc::now(),
                };
                kterm::store::index::create_entry(&mut app.index, &id, entry)?;
                kterm::store::index::save(&app.config.index_path(), &app.index)?;

                // Scratchから昇格の場合は削除
                if matches!(app.tab, Tab::Scratch) && let Some(scratch) = app.scratches.get(app.selected_index) {
                    let timestamp = scratch.timestamp.clone();
                    kterm::store::scratch::delete(&app.config.scratch_dir(), &timestamp)?;
                    app.scratches.remove(app.selected_index);
                }

                app.zettels.push(zettel);
                app.zettels.sort_by(|a, b| a.id.cmp(&b.id));
            }
            app.close_wizard();
        }
        KeyCode::Char(c) => {
            if let Some(state) = &mut app.wizard_state {
                match state.focused_field {
                    WizardField::Id => state.id.push(c),
                    WizardField::Title => state.title.push(c),
                    WizardField::Tags => state.tags.push(c),
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(state) = &mut app.wizard_state {
                match state.focused_field {
                    WizardField::Id => {
                        state.id.pop();
                    }
                    WizardField::Title => {
                        state.title.pop();
                    }
                    WizardField::Tags => {
                        state.tags.pop();
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_tag_edit(app: &mut App, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => app.close_tag_edit(),
        KeyCode::Enter => {
            // タグを保存
            if let Some(state) = &app.tag_edit_state {
                let tags: Vec<String> = state
                    .input
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();

                // Zettelのタグを更新
                if let Some(zettel) = app.zettels.get_mut(app.selected_index) {
                    zettel.tags = tags.clone();
                    let id = zettel.id.clone();

                    // index.jsonを更新
                    if let Ok(entry) = kterm::store::index::read_entry(&app.index, &id) {
                        let updated_entry = kterm::model::index::IndexEntry {
                            status: kterm::model::index::CardStatus::Permanent,
                            tags,
                            created: entry.created,
                            updated: chrono::Utc::now(),
                        };
                        kterm::store::index::update_entry(&mut app.index, &id, updated_entry)?;
                        kterm::store::index::save(&app.config.index_path(), &app.index)?;
                    }
                }
            }
            app.close_tag_edit();
        }
        KeyCode::Char(c) => {
            if let Some(state) = &mut app.tag_edit_state {
                state.input.push(c);
            }
        }
        KeyCode::Backspace => {
            if let Some(state) = &mut app.tag_edit_state {
                state.input.pop();
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_confirm_delete(app: &mut App, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.confirm_delete()?;
        }
        _ => {
            app.cancel_delete();
        }
    }
    Ok(())
}

fn open_editor(
    terminal: &mut ratatui::Terminal<CrosstermBackend<io::Stdout>>,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ratatuiを一時停止
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // エディタを起動
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    std::process::Command::new(editor).arg(path).status()?;

    // Ratatuiを再開
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.clear()?;

    Ok(())
}

fn handle_workspace_switch(app: &mut App, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => app.close_workspace_switch(),
        KeyCode::Char('j') | KeyCode::Down => app.workspace_switch_move(1),
        KeyCode::Char('k') | KeyCode::Up => app.workspace_switch_move(-1),
        KeyCode::Enter => app.confirm_workspace_switch()?,
        _ => {}
    }
    Ok(())
}

fn handle_help(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('?') => app.close_help(),
        _ => {}
    }
}
