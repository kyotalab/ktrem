use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect) {
    let key_style = Style::default().fg(Color::Yellow);
    let desc_style = Style::default().fg(Color::White);

    let lines = vec![
        Line::from(Span::styled("Navigation", Style::default().fg(Color::Cyan))),
        help_line("j / k / ↓ / ↑", "Move down / up", key_style, desc_style),
        help_line(
            "h / l / ← / →",
            "Collapse / expand (Zettelkasten only)",
            key_style,
            desc_style,
        ),
        help_line(
            "Tab",
            "Switch tab (Scratch / Zettelkasten)",
            key_style,
            desc_style,
        ),
        Line::from(""),
        Line::from(Span::styled("Notes", Style::default().fg(Color::Cyan))),
        help_line("Enter", "Open note in $EDITOR", key_style, desc_style),
        help_line(
            "n",
            "New note (Scratch: instant, Zettelkasten: wizard)",
            key_style,
            desc_style,
        ),
        help_line(
            "p",
            "Promote Scratch to Zettelkasten",
            key_style,
            desc_style,
        ),
        help_line("t", "Edit tags (Zettelkasten only)", key_style, desc_style),
        help_line("d", "Delete (with confirmation)", key_style, desc_style),
        Line::from(""),
        Line::from(Span::styled(
            "Search & Workspace",
            Style::default().fg(Color::Cyan),
        )),
        help_line(
            "/",
            "Incremental search (Enter to confirm, Esc to cancel)",
            key_style,
            desc_style,
        ),
        help_line("w", "Switch workspace", key_style, desc_style),
        Line::from(""),
        Line::from(Span::styled("Other", Style::default().fg(Color::Cyan))),
        help_line("?", "Toggle this help", key_style, desc_style),
        help_line("q / Esc", "Quit (Normal mode only)", key_style, desc_style),
    ];

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Help"));

    frame.render_widget(paragraph, area);
}

fn help_line<'a>(key: &'a str, desc: &'a str, key_style: Style, desc_style: Style) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("  {:<20}", key), key_style),
        Span::styled(desc, desc_style),
    ])
}
