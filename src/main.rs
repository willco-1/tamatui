//! # [Ratatui] Canvas example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::time::{Duration, Instant};

use color_eyre::{eyre::bail, Result};
use ratatui::{prelude::Alignment,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    widgets::{Paragraph,
        canvas::{Canvas, Circle, Map, MapResolution, Rectangle},
        Block, Widget,
    },
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    pet_position: (f64, f64), // x, y coordinates of our pet
    playground: Rect,
    hunger: u32, // A simple metric for the pet's needs
    happiness: u32,
    tick_count: u64,
    marker: Marker,
}
impl App {
    fn new() -> Self {
        Self {
            pet_position: (100.0, 50.0), // Start in the middle of the playground
            playground: Rect::new(10, 10, 200, 100),
            hunger: 0,
            happiness: 100,
            tick_count: 0,
            marker: Marker::Braille, // Start with Braille for detailed representation
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| self.draw(frame))?;
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.pet_position.1 += 1.0;
                        // Ensure the pet doesn't move out of the playground
                        self.pet_position.1 = self.pet_position.1.min(self.playground.bottom() as f64 - 5.0);
                    },
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.pet_position.1 -= 1.0;
                        self.pet_position.1 = self.pet_position.1.max(self.playground.top() as f64);
                    },
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.pet_position.0 += 1.0;
                        self.pet_position.0 = self.pet_position.0.min(self.playground.right() as f64 - 5.0);
                    },
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.pet_position.0 -= 1.0;
                        self.pet_position.0 = self.pet_position.0.max(self.playground.left() as f64);
                    },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            self.on_tick();
            last_tick = Instant::now();
        }
    }
}

    fn on_tick(&mut self) {
    self.tick_count += 1;

    // Increase hunger over time
    if self.tick_count % 60 == 0 { // Roughly every second if tick_rate is 16ms
        self.hunger = (self.hunger + 1).min(100);
    }

    // Decrease happiness over time
    if self.tick_count % 120 == 0 {
        self.happiness = self.happiness.saturating_sub(1);
    }

    // Update marker for visual change
    if (self.tick_count % 180) == 0 {
        self.marker = match self.marker {
            Marker::Dot => Marker::Braille,
            Marker::Braille => Marker::Block,
            Marker::Block => Marker::HalfBlock,
            Marker::HalfBlock => Marker::Bar,
            Marker::Bar => Marker::Dot,
        };
    }

    // Simple pet movement logic (could be expanded for more complex behavior)
    let (dx, dy) = (1.0, -0.5); // Example movement vector
    let new_x = self.pet_position.0 + dx;
    let new_y = self.pet_position.1 + dy;

    // Keep the pet within bounds
    self.pet_position.0 = new_x.max(self.playground.left() as f64).min(self.playground.right() as f64 - 5.0);
    self.pet_position.1 = new_y.max(self.playground.top() as f64).min(self.playground.bottom() as f64 - 5.0);
}
      fn pet_canvas(&self) -> impl Widget + '_ {
    Canvas::default()
        .block(Block::bordered().title("Tamagotchi"))
        .marker(self.marker)
        .paint(|ctx| {
            // Draw the pet - this can be made more complex
            ctx.draw(&Circle {
                x: self.pet_position.0,
                y: self.pet_position.1,
                radius: 5.0,
                color: Color::Yellow,
            });
            // Maybe add eyes or a smile to indicate mood
        })
        .x_bounds([10.0, 210.0])
        .y_bounds([10.0, 110.0])
}

fn draw(&self, frame: &mut Frame) {
        let sizes = Layout::horizontal([
            Constraint::Percentage(30), // Smaller percentage for status
            Constraint::Percentage(70), // Larger for pet area
        ]).split(frame.area());
        let [status, pet_area] = *sizes else { todo!() };


        frame.render_widget(self.status_canvas(), status);
        frame.render_widget(self.pet_canvas(), pet_area);
    }

fn status_canvas(&self) -> impl Widget {
    let text = vec![
        format!("Hunger: {}", self.hunger),
        format!("Happiness: {}", self.happiness),
    ];
    Paragraph::new(text.join("\n"))
        .block(Block::bordered().title("Status").style(Style::default().fg(Color::White)))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
}    fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height).mul_add(2.0, -4.0);
        Canvas::default()
            .block(Block::bordered().title("Rects"))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                for i in 0..=11 {
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 2.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Red,
                    });
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 21.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Blue,
                    });
                }
                for i in 0..100 {
                    if i % 10 != 0 {
                        ctx.print(f64::from(i) + 1.0, 0.0, format!("{i}", i = i % 10));
                    }
                    if i % 2 == 0 && i % 10 != 0 {
                        ctx.print(0.0, f64::from(i), format!("{i}", i = i % 10));
                    }
                }
            })
    }
}

