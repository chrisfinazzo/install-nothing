use colored::*;
use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Free,
    Used,
    Fragmented,
    Unmovable,
    Bad,
    Reading,
    Writing,
}

impl Cell {
    fn glyph(self) -> char {
        match self {
            Cell::Free => '·',
            Cell::Used => '█',
            Cell::Fragmented => '▓',
            Cell::Unmovable => '▒',
            Cell::Bad => 'X',
            Cell::Reading => 'R',
            Cell::Writing => 'W',
        }
    }

    fn paint(self, run: &str) -> ColoredString {
        match self {
            Cell::Free => run.dimmed(),
            Cell::Used => run.bright_green(),
            Cell::Fragmented => run.yellow(),
            Cell::Unmovable => run.bright_blue(),
            Cell::Bad => run.bright_red().bold(),
            Cell::Reading => run.bright_cyan().bold(),
            Cell::Writing => run.bright_magenta().bold(),
        }
    }

    pub fn is_data(self) -> bool {
        matches!(self, Cell::Used | Cell::Fragmented)
    }

    pub fn legend(self) -> String {
        format!("{} {}", self.paint(&self.glyph().to_string()), self.label())
    }

    fn label(self) -> &'static str {
        match self {
            Cell::Free => "Free",
            Cell::Used => "Used",
            Cell::Fragmented => "Fragmented",
            Cell::Unmovable => "Unmovable",
            Cell::Bad => "Bad",
            Cell::Reading => "Reading",
            Cell::Writing => "Writing",
        }
    }
}

pub struct BlockGrid {
    width: usize,
    cells: Vec<Cell>,
    last_frame: usize,
}

impl BlockGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            cells: vec![Cell::Free; width * height],
            last_frame: 0,
        }
    }

    pub fn fit(max_width: usize, height: usize) -> Self {
        let columns = match terminal::size() {
            Ok((columns, _)) if columns > 0 => columns as usize,
            _ => 80,
        };
        let width = columns.saturating_sub(4).clamp(20, max_width);

        Self::new(width, height)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, index: usize) -> Cell {
        self.cells[index]
    }

    pub fn set(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }

    pub fn find(&self, from: usize, predicate: impl Fn(Cell) -> bool) -> Option<usize> {
        (from..self.cells.len()).find(|i| predicate(self.cells[*i]))
    }

    pub fn rfind(&self, before: usize, predicate: impl Fn(Cell) -> bool) -> Option<usize> {
        (0..before.min(self.cells.len()))
            .rev()
            .find(|i| predicate(self.cells[*i]))
    }

    fn render_row(row: &[Cell]) -> String {
        let mut out = String::new();
        let mut run = String::new();
        let mut current = row[0];

        for cell in row {
            if *cell != current {
                out.push_str(&current.paint(&run).to_string());
                run.clear();
                current = *cell;
            }
            run.push(cell.glyph());
        }
        out.push_str(&current.paint(&run).to_string());

        out
    }

    pub fn draw(&mut self, footer: &[String]) -> io::Result<()> {
        let stdout = io::stdout();
        let mut out = stdout.lock();

        if self.last_frame > 0 {
            queue!(out, cursor::MoveToPreviousLine(self.last_frame as u16))?;
        }

        let mut lines = 0;
        for row in self.cells.chunks(self.width) {
            queue!(out, terminal::Clear(ClearType::CurrentLine))?;
            writeln!(out, "  {}", Self::render_row(row))?;
            lines += 1;
        }
        for line in footer {
            queue!(out, terminal::Clear(ClearType::CurrentLine))?;
            writeln!(out, "{}", line)?;
            lines += 1;
        }
        out.flush()?;

        self.last_frame = lines;
        Ok(())
    }

    pub fn finish(&mut self) {
        self.last_frame = 0;
    }
}

pub struct HiddenCursor;

impl HiddenCursor {
    pub fn new() -> Self {
        let _ = execute!(io::stdout(), cursor::Hide);
        Self
    }
}

impl Drop for HiddenCursor {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show);
    }
}
