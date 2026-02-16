use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{error::Error, io};

struct App {
    name: String,
    counter: i32,
    input_mode: bool,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            name: "Мир".to_string(),
            counter: 0,
            input_mode: false,
            messages: vec!["Добро пожаловать в Rust TUI приложение!".to_string()],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Настройка терминала
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Создание приложения
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    // Восстановление терминала
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.input_mode {
                match key.code {
                    KeyCode::Enter => {
                        app.messages.push(format!("Привет, {}! Добро пожаловать!", app.name));
                        app.input_mode = false;
                    }
                    KeyCode::Char(c) => {
                        app.name.push(c);
                    }
                    KeyCode::Backspace => {
                        app.name.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = false;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => {
                        app.input_mode = true;
                        app.name.clear();
                    }
                    KeyCode::Char('+') => {
                        app.counter += 1;
                        app.messages.push(format!("Счетчик увеличен до {}", app.counter));
                    }
                    KeyCode::Char('-') => {
                        app.counter -= 1;
                        app.messages.push(format!("Счетчик уменьшен до {}", app.counter));
                    }
                    KeyCode::Char('r') => {
                        app.counter = 0;
                        app.messages.push("Счетчик сброшен до 0".to_string());
                    }
                    KeyCode::Char('c') => {
                        app.messages.clear();
                        app.messages.push("История очищена".to_string());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3), 
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(f.area());

    // Заголовок
    let title = Paragraph::new("🦀 Простое GUI приложение на Rust 🦀")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(title, chunks[0]);

    // Информация о пользователе и счетчике
    let info_text = if app.input_mode {
        vec![
            Line::from(vec![
                Span::styled("Режим ввода: ", Style::default()),
                Span::styled(&app.name, Style::default().fg(Color::Yellow)),
                Span::styled(" (Enter - подтвердить, Esc - отмена)", Style::default().fg(Color::Gray)),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Имя: ", Style::default()),
                Span::styled(&app.name, Style::default().fg(Color::Green)),
                Span::styled(" | Счетчик: ", Style::default()),
                Span::styled(app.counter.to_string(), Style::default().fg(Color::Magenta)),
            ]),
        ]
    };

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Информация"));
    f.render_widget(info, chunks[1]);

    // Панель управления
    let controls = Paragraph::new(Text::from(vec![
        Line::from("Управление:"),
        Line::from("n - изменить имя | + - увеличить счетчик | - - уменьшить"),
        Line::from("r - сбросить счетчик | c - очистить историю | q - выход"),
    ]))
    .style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL).title("Команды"))
    .wrap(Wrap { trim: true });
    f.render_widget(controls, chunks[2]);

    // История сообщений
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::styled(
                format!("{}: {}", i + 1, m),
                Style::default().fg(Color::White),
            ));
            ListItem::new(content)
        })
        .collect();

    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("История"))
        .style(Style::default().fg(Color::White));
    f.render_widget(messages_list, chunks[3]);

    // Показать счетчик как прогресс-бар
    if app.counter > 0 {
        let progress = (app.counter.abs() as f64 / 100.0).min(1.0);
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Прогресс"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((progress * 100.0) as u16);
        
        let popup_area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, popup_area);
        f.render_widget(gauge, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
