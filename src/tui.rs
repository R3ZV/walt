use std::io;

use crate::setters;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::{thread_rng, Rng};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};

#[derive(Debug)]
pub struct App {
    platform: String,
    wallpapers: Wallpapers,
    should_exit: bool,
}

#[derive(Debug)]
struct Wallpapers {
    items: Vec<Wallpaper>,
    state: ListState,
}

#[derive(Debug)]
struct Wallpaper {
    path: String,
    name: String,
}

impl Wallpapers {
    fn new(items: Vec<Wallpaper>) -> Self {
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl Wallpaper {
    fn new(path: String) -> Self {
        let name = if let Some(pos) = path.rfind('/') {
            path[pos + 1..].to_string()
        } else {
            path.to_string()
        };

        Self { path, name }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, preview_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_preview(preview_area, buf);
    }
}

impl App {
    pub fn new(wallpaper_dir: &str, platform: &str) -> Self {
        let images: Vec<Wallpaper> = setters::read_wallpapers(wallpaper_dir)
            .unwrap()
            .iter()
            .map(|path| Wallpaper::new(path.clone()))
            .collect();

        let should_exit = false;
        let wallpapers = Wallpapers::new(images);

        Self {
            platform: platform.to_string(),
            wallpapers,
            should_exit,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Walt").bold().centered().render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "Use j/k to move, Enter to set wallpaper, Q / ESC to exit, r to select random, g/G to go top/bottom.",
        )
        .centered()
        .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Images").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN);

        let items: Vec<ListItem> = self
            .wallpapers
            .items
            .iter()
            .map(|w| ListItem::from(format!("{}", w.name)))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.wallpapers.state);
    }

    fn render_preview(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Preview").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN);

        Widget::render(block, area, buf);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') => self.select_first(),
            KeyCode::Char('G') => self.select_last(),
            KeyCode::Char('r') => self.select_and_set_random(),
            KeyCode::Enter => self.set_wallpaper(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.wallpapers.state.select_next();
    }

    fn select_previous(&mut self) {
        self.wallpapers.state.select_previous();
    }

    fn select_first(&mut self) {
        self.wallpapers.state.select_first();
    }

    fn select_last(&mut self) {
        self.wallpapers.state.select_last();
    }

    fn set_wallpaper(&mut self) {
        if let Some(i) = self.wallpapers.state.selected() {
            setters::set_wallpaper(&self.wallpapers.items[i].path, self.platform.as_str());
        }
    }

    fn select_and_set_random(&mut self) {
        let mut rng = thread_rng();
        let idx: usize = rng.gen_range(0..self.wallpapers.items.len());
        self.wallpapers.state.select(Some(idx));
        self.set_wallpaper();
    }
}
