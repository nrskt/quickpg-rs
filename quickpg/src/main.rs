extern crate postgres;
extern crate quickpg;
extern crate termion;
extern crate tui;

use postgres::{Connection, TlsMode};
use quickpg::model::{PostgresTable, TableList};

use std::io;
use std::iter;
use std::str::SplitTerminator;
use std::sync::mpsc;
use std::thread;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Size};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Widget};
use tui::Terminal;

struct App<'a> {
    items: Vec<PostgresTable>,
    selected: usize,
    input: String,
    query_result: Vec<String>,
    pg_conn: &'a Connection,
}

impl<'a> App<'a> {
    fn new(conn: &Connection) -> App {
        let tbl_list = TableList::new(&conn);
        App {
            items: tbl_list.tables,
            selected: 0,
            input: String::new(),
            query_result: Vec::new(),
            pg_conn: conn,
        }
    }
}

enum Event {
    Input(event::Key),
}

fn main() {
    let conn = Connection::connect(
        "postgresql://postgres@localhost:5432/curama.local",
        TlsMode::None,
    ).unwrap();
    let mut app = App::new(&conn);

    let mut terminal = init().expect("Fail Initialization");
    terminal.clear().unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    draw(&mut terminal, &app);

    loop {
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Char('\n') => {
                    run_query(&mut app);
                }
                event::Key::Char(c) => {
                    app.input.push(c);
                }
                event::Key::Backspace => {
                    app.input.pop();
                }
                event::Key::Down => {
                    app.selected += 1;
                    if app.selected > app.items.len() - 1 {
                        app.selected = 0;
                    }
                }
                event::Key::Up => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.items.len() - 1;
                },
                _ => {}
            },
        }
        draw(&mut terminal, &app);
    }
}

fn init() -> Result<Terminal<MouseBackend>, io::Error> {
    let backend = MouseBackend::new()?;
    Terminal::new(backend)
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    let size = t.size().unwrap();
    let tables: Vec<String> = app.items.iter().map(|tbl| tbl.fullname()).collect();
    let tables: Vec<String> = tables
        .iter()
        .filter(|tblname| tblname.to_lowercase().as_str().contains(app.input.as_str()))
        .map(|tbl| tbl.to_string())
        .collect();

    let dataset: Vec<Vec<String>> = app
        .query_result
        .iter()
        .map(|item| item.split_terminator(',').map(|s| s.to_string()).collect())
        .collect();
    let dataset: Vec<_> = dataset.iter().map(|v| Row::Data(v.iter())).collect();
    let fields_count = dataset.len();
    let widths_size: Vec<u16> = vec![20; fields_count];

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(10), Size::Percent(50), Size::Percent(40)])
        .render(t, &size, |t, chunks| {
            Paragraph::default()
                .block(Block::default().title("Filter").borders(Borders::ALL))
                .wrap(true)
                .text(&app.input)
                .render(t, &chunks[0]);

            let style = Style::default().fg(Color::White).bg(Color::Black);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Tables"))
                .items(&tables)
                .select(app.selected)
                .style(style)
                .highlight_style(style.clone().fg(Color::LightGreen).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(t, &chunks[1]);

            let row_style = Style::default().fg(Color::White);
            Table::new(vec!["dataset"].into_iter(), dataset.into_iter())
                .block(Block::default().title("Table"))
                .widths(&widths_size)
                .render(t, &chunks[2]);
        });

    t.draw().unwrap()
}

fn run_query(app: &mut App) {
    let selected_table = app.items.get(app.selected).unwrap();
    let result = selected_table.get(app.pg_conn);
    println!("{:?}", result.len());
    app.query_result = result;
}

fn clear_query_result(t: &mut Terminal<MouseBackend>, app: &mut App) {
    app.query_result = Vec::new();
    draw(t, app);
}
