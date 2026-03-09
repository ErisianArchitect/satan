use std::io::{Stdout, Write, stdout};
use std::time::Duration;
use crossterm::cursor::MoveTo;
use ratatui::Terminal;
use crossterm::{QueueableCommand, event};
use crossterm::{execute};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TestWidget(&'static str);

impl ratatui::widgets::Widget for TestWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        for y in area.y..area.bottom() {
            for x in area.x..area.right() {
                let Some(cell) = buf.cell_mut((x, y)) else {
                    continue;
                };
                cell.bg = Color::Indexed(49);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let mut term = Terminal::<ratatui::backend::CrosstermBackend<Stdout>>::new(ratatui::backend::CrosstermBackend::new(stdout))?;
    let test_widget = TestWidget("hello, world.");
    loop {
        let redraw = match event::read()? {
            event::Event::FocusGained => true,
            event::Event::FocusLost => true,
            event::Event::Key(key_event) => {
                true
            },
            event::Event::Mouse(mouse_event) => {
                true
            },
            event::Event::Paste(_) => {
                true
            },
            event::Event::Resize(_, _) => {
                true
            },
        };
        if redraw {
            term.draw(|frame| {
                frame.render_widget(test_widget, frame.area());
            })?;
        }
    }

    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}