// pub struct Field<const R: usize, const C: usize>([[bool; C]; R]);

// Don't need to fully parse to traverse the tree
pub struct Field<'a>(&'a str);

// impl<const R: usize, const C: usize> Field<R, C> {
impl<'a> Field<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    /// Traverse the field using a iterator cycle to increment columns
    pub fn traverse_cycle(&self, rise: usize, run: usize) -> usize {
        let mut trees = 0;
        let mut col = 0;

        for row in self.0.lines().step_by(rise) {
            if row.chars().cycle().nth(col) == Some('#') {
                trees += 1;
            }
            col += run
        }

        trees
    }

    /// Traverse the field using a vector with modulo index to increment columns
    pub fn traverse_collect(&self, rise: usize, run: usize) -> usize {
        let mut trees = 0;
        let mut col = 0;

        for row in self.0.lines().step_by(rise) {
            let chars = row.chars().collect::<Vec<_>>();
            if chars[(col % chars.len())] == '#' {
                trees += 1;
            }
            col += run
        }

        trees
    }
}
