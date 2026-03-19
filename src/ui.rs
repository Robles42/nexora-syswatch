use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph, List, ListItem},
    style::{Color, Style},
    Frame,
};
use crate::monitor::SystemStats;

pub fn render(f: &mut Frame, stats: &SystemStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // CPU
            Constraint::Length(3), // RAM
            Constraint::Min(7),    // Procesos
        ].as_ref())
        .split(f.size());

    // Header
    let title = Paragraph::new(format!(" [ nexora-syswatch ] | Host: {} ", stats.host_name))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
        );
    f.render_widget(title, chunks[0]);

    // CPU Gauge - Cambia a rojo si el uso es alto
    let cpu_color = if stats.cpu_usage > 80.0 { Color::Red } else { Color::Yellow };
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(" CPU Usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(cpu_color))
        .percent(stats.cpu_usage as u16);
    f.render_widget(cpu_gauge, chunks[1]);

    // RAM Gauge
    let mem_percent = (stats.mem_used as f64 / stats.mem_total as f64 * 100.0) as u16;
    let mem_gauge = Gauge::default()
        .block(Block::default().title(" RAM Usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .label(format!("{}/{} MB", stats.mem_used, stats.mem_total))
        .percent(mem_percent);
    f.render_widget(mem_gauge, chunks[2]);

    // Lista de Procesos (Top 5 RAM)
    let items: Vec<ListItem> = stats.top_processes.iter()
        .map(|p| ListItem::new(format!(" > {:<20} | {:>6} MB", p.name, p.mem_usage)))
        .collect();

    let process_list = List::new(items)
        .block(Block::default()
            .title(" Top Processes (RAM) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(process_list, chunks[3]);
}