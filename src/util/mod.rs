#![allow(warnings)]

mod event;
mod sort;
mod add;

pub use event::{ Event, Events };
pub use sort::{ SortBy, SortDirection };
pub use add::Addby;
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    Console,
    Main,
}