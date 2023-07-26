#[derive(Debug)]
pub struct Liner<'a> {
    lines: Vec<&'a str>,
    pub position: usize,
    pub shift: usize, // shift - const field
}

impl<'a> Liner<'a> {
    pub fn new(content: &'a str, shift: usize) -> Self {
        let lines: Vec<&'a str> = content.lines().collect();

        Self {
            position: 0,
            lines,
            shift,
        }
    }

    pub fn scroll_up(&mut self) {
        self.position = self.position.checked_sub(1).unwrap_or(0);
    }

    pub fn scroll_up_to(&mut self, to: usize) {
        self.position = self.position.checked_sub(to).unwrap_or(0);
    }

    pub fn scroll_down(&mut self) {
        self.position = if self.position + 1 == self.lines.len() {
            self.position
        } else {
            self.position + 1
        };
    }

    pub fn scroll_down_to(&mut self, to: usize) {
        self.position = if self.position + to == self.lines.len() {
            self.position
        } else {
            self.position + to
        };
    }

    pub fn get_current_lines(&self) -> Vec<Option<&'a str>> {
        (self.position..self.position + self.shift)
            .map(|i| self.lines.get(i).map(|a| *a))
            .collect()
    }
}
