use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use crate::services::telemetry;

pub async fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stats = telemetry::RealtimeStats {
        cpu_usage: 0.0,
        mem_usage_mb: 0,
        active_regions: 0,
        latest_log: "Connecting to telemetry stream...".to_string(),
    };

    use tokio_stream::StreamExt;
    let mut stream = telemetry::stream_realtime_stats().await?;

    loop {
        tokio::select! {
            Some(res) = stream.next() => {
                if let Ok(new_stats) = res {
                    stats = new_stats;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {}
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let block1 = Block::default().title("Vyzorix Edge Realtime Logs").borders(Borders::ALL);
            let log_msg = format!("Press 'q' to exit...\n{}", stats.latest_log);
            let paragraph1 = Paragraph::new(log_msg).block(block1);
            f.render_widget(paragraph1, chunks[0]);

            let block2 = Block::default().title("System Telemetry").borders(Borders::ALL);
            let stats_msg = format!(
                "CPU: {:.1}% | RAM: {}MB / 1024MB\nEdge Regions Online: {}", 
                stats.cpu_usage, stats.mem_usage_mb, stats.active_regions
            );
            let paragraph2 = Paragraph::new(stats_msg).block(block2);
            f.render_widget(paragraph2, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
