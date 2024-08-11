use std::io::stdout;
use std::io::Result as IOResult;
use std::time::{Duration, Instant};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Paragraph},
    Terminal,
};
use ratatui::prelude::*;

mod flag_config;

use flag_config::*;

#[derive(Debug)]
struct RenderConfig {
    show_flag_name: bool,
    show_color_names: bool,
}

fn render_flag(frame: &mut Frame, flag: &PrideFlag, config: &RenderConfig) {
    let area = frame.size();

    let colors: Vec<Color> = flag
        .colors
        .iter()
        .map(|c| Color::Rgb(c.r, c.g, c.b))
        .collect();

    let color_count: u32 = colors.len().try_into().unwrap();

    let mut constraints = vec![];

    for _ in 0..colors.len() {
        constraints.push(Constraint::Ratio(1,color_count));
    }

    let layout = Layout::new(
        Direction::Horizontal,
        constraints,
    )
        .split(area);

    for (index, color) in colors.iter().enumerate() {
        let color_bar = Paragraph::new("")
            .style(Style::default().bg(*color));

        frame.render_widget(color_bar, layout[index]);
    }

    if config.show_flag_name {
        let fg = if let Some(Color::Rgb(r, g, b)) = colors.first() {
            Color::Rgb(255-r, 255-g, 255-b)
        } else {
            Color::Black
        };

        let style = Style::new().fg(fg);
        let name: Span = flag.name.clone().into();
        let name = name.style(style);

        let bottom_left_corner = Rect {
            y: layout[0].y + layout[0].height - 1,
            ..layout[0]
        };

        frame.render_widget(name, bottom_left_corner);
    }

    if config.show_color_names {
        for (index, color_config) in flag.colors.iter().enumerate() {
            let fg = if let Some(Color::Rgb(r, g, b)) = colors.get(index) {
                Color::Rgb(255-r, 255-g, 255-b)
            } else {
                Color::Black
            };

            let style = Style::new().fg(fg);
            let name: Span = color_config.name.clone().into();
            let name = name.style(style);
            let name_width: u16 = name.width().try_into().unwrap();

            if let Some(stripe_layout) = layout.get(index) {
                let bottom_left_corner = Rect {
                    x: stripe_layout.x + (stripe_layout.width - name_width),
                    y: stripe_layout.y + stripe_layout.height - 1,
                    ..*stripe_layout
                };

                frame.render_widget(name, bottom_left_corner);
            }
        }
    }
}

fn main() -> IOResult<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let flag_config = parse_flags();
    let mut render_config = RenderConfig {
        show_flag_name: flag_config.show_flag_name,
        show_color_names: flag_config.show_color_names,
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut flag_index: usize = 0;

    let mut start = Instant::now();

    let mut advance_flag = false;

    loop {
        let now = Instant::now();

        terminal.draw(|frame| {
            if let Some(flag) = flag_config.flags.get(flag_index) {
                render_flag(frame, flag, &render_config);

                let delta = start.elapsed();

                if delta.as_secs() > flag_config.rotation_delay_seconds.try_into().unwrap() {
                    advance_flag = true;
                }
            }
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                    advance_flag = true;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('t') {
                    flag_index = 0; // transgender
                    advance_flag = false;
                    start = now;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('b') {
                    flag_index = 1; // bisexual
                    advance_flag = false;
                    start = now;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                    flag_index = 2; // genderqueer
                    advance_flag = false;
                    start = now;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('p') {
                    flag_index = 3; // pansexual
                    advance_flag = false;
                    start = now;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('f') {
                    render_config.show_flag_name = !render_config.show_flag_name;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('c') {
                    render_config.show_color_names = !render_config.show_color_names;
                }
            }
        }

        if advance_flag {
            flag_index += 1;

            if flag_index == flag_config.flags.len() {
                flag_index = 0;
            }

            start = now;
        }

        advance_flag = false;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
