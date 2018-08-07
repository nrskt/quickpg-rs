extern crate postgres;
extern crate quickpg;
extern crate quickpg_ui;
extern crate termion;
extern crate tui;

use std::io;
use std::process;
use std::sync::mpsc;
use std::thread;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::Terminal;

use quickpg_ui::ui::main_widget::UiContext;
use quickpg_ui::ui::status_bar::Mode;

enum Event {
    Input(event::Key),
}

fn main() {
    let mut cntx = UiContext::new();

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

    cntx.draw_main(&mut terminal);

    loop {
        let evt = rx.recv().unwrap();
        match cntx.status_bar.status {
            Mode::Normal => normalmode_dispatch(&mut cntx, evt),
            Mode::Filter => filtermode_dispatch(&mut cntx, evt),
        }
        cntx.draw_main(&mut terminal);
    }
}

fn filtermode_dispatch(cntx: &mut UiContext, evt: Event) {
    match evt {
        Event::Input(input) => match input {
            event::Key::Down => {
                cntx.pg_table_list.next();
            }
            event::Key::Up => {
                cntx.pg_table_list.prev();
            }
            event::Key::Char('\n') => {
                cntx.status_bar.toggled_filter();
            }
            event::Key::Char(c) => {
                cntx.filter_box.push(c);
                cntx.pg_table_list.push(c);
            }
            event::Key::Backspace => {
                cntx.filter_box.pop();
                cntx.pg_table_list.pop();
            }
            event::Key::F(4) => {
                cntx.status_bar.toggled_filter();
            }
            _ => {}
        },
    }
}

fn normalmode_dispatch(cntx: &mut UiContext, evt: Event) {
    match evt {
        Event::Input(input) => match input {
            event::Key::Char('q') => {
                process::exit(0);
            }
            event::Key::Down => {
                cntx.pg_table_list.next();
            }
            event::Key::Up => {
                cntx.pg_table_list.prev();
            }
            event::Key::F(4) => {
                cntx.status_bar.toggled_filter();
            }
            _ => {}
        },
    }
}

fn init() -> Result<Terminal<MouseBackend>, io::Error> {
    let backend = MouseBackend::new()?;
    Terminal::new(backend)
}
