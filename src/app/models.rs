use tui::widgets::{ListState, TableState};

use super::Route;

pub trait Scrollable {
    fn handle_scroll(&mut self, up: bool, page: bool) {
        // support page up/down
        let inc_or_dec = if page { 10 } else { 1 };
        if up {
            self.scroll_up(inc_or_dec);
        } else {
            self.scroll_down(inc_or_dec);
        }
    }
    fn scroll_down(&mut self, inc_or_dec: usize);
    fn scroll_up(&mut self, inc_or_dec: usize);
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    // pub fn with_items(items: Vec<T>) -> StatefulList<T> {
    //     let mut state = ListState::default();
    //     if !items.is_empty() {
    //         state.select(Some(0));
    //     }
    //     StatefulList { state, items }
    // }
}

impl<T> Scrollable for StatefulList<T> {
    // for lists we cycle back to the beginning when we reach the end
    fn scroll_down(&mut self, increment: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(increment) {
                    0
                } else {
                    i + increment
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    // for lists we cycle back to the end when we reach the beginning
    fn scroll_up(&mut self, decrement: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(decrement)
                } else {
                    i.saturating_sub(decrement)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Clone)]
pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
    pub fn new() -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        let mut table = StatefulTable::new();
        if !items.is_empty() {
            table.state.select(Some(0));
        }
        table.set_items(items);
        table
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        let item_len = items.len();
        self.items = items;
        if !self.items.is_empty() {
            let i = self.state.selected().map_or(0, |i| {
                if i > 0 && i < item_len {
                    i
                } else if i >= item_len {
                    item_len - 1
                } else {
                    0
                }
            });
            self.state.select(Some(i));
        }
    }
}

impl<T> Scrollable for StatefulTable<T> {
    fn scroll_down(&mut self, increment: usize) {
        if let Some(i) = self.state.selected() {
            if (i + increment) < self.items.len() {
                self.state.select(Some(i + increment));
            } else {
                self.state.select(Some(self.items.len().saturating_sub(1)));
            }
        }
    }

    fn scroll_up(&mut self, decrement: usize) {
        if let Some(i) = self.state.selected() {
            if i != 0 {
                self.state.select(Some(i.saturating_sub(decrement)));
            }
        }
    }
}

impl<T: Clone> StatefulTable<T> {
    /// a clone of the currently selected item.
    /// for mutable ref use state.selected() and fetch from items when needed
    pub fn get_selected_item_copy(&self) -> Option<T> {
        if !self.items.is_empty() {
            self.state.selected().map(|i| self.items[i].clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct TabRoute {
    pub title: String,
    pub route: Route,
}

pub struct TabsState {
    pub items: Vec<TabRoute>,
    pub index: usize,
}

impl TabsState {
    pub fn new(items: Vec<TabRoute>) -> TabsState {
        TabsState { items, index: 0 }
    }
    pub fn set_index(&mut self, index: usize) -> &TabRoute {
        self.index = index;
        &self.items[self.index]
    }
    // pub fn get_active_route(&self) -> &Route {
    //     &self.items[self.index].route
    // }

    // pub fn next(&mut self) {
    //     self.index = (self.index + 1) % self.items.len();
    // }
    // pub fn previous(&mut self) {
    //     if self.index > 0 {
    //         self.index -= 1;
    //     } else {
    //         self.index = self.items.len() - 1;
    //     }
    // }
}

#[derive(Debug, PartialEq)]
pub struct ScrollableTxt {
    items: Vec<String>,
    pub offset: u16,
}

impl ScrollableTxt {
    // pub fn new() -> ScrollableTxt {
    //     ScrollableTxt {
    //         items: vec![],
    //         offset: 0,
    //     }
    // }
    //
    // pub fn with_string(item: String) -> ScrollableTxt {
    //     let items: Vec<&str> = item.split('\n').collect();
    //     let items: Vec<String> = items.iter().map(|it| it.to_string()).collect();
    //     ScrollableTxt { items, offset: 0 }
    // }

    // pub fn get_txt(&self) -> String {
    //     self.items.join("\n")
    // }
}

impl Scrollable for ScrollableTxt {
    fn scroll_down(&mut self, increment: usize) {
        // scroll only if offset is less than total lines in text
        // we subtract increment + 2 to keep the text in view. Its just an arbitrary number that works
        if self.offset < self.items.len().saturating_sub(increment + 2) as u16 {
            self.offset += increment as u16;
        }
    }
    fn scroll_up(&mut self, decrement: usize) {
        // scroll up and avoid going negative
        if self.offset > 0 {
            self.offset = self.offset.saturating_sub(decrement as u16);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_table_scroll() {
        let mut item: StatefulTable<&str> = StatefulTable::new();
        item.set_items(vec!["A", "B", "C"]);

        assert_eq!(item.state.selected(), Some(0));

        item.handle_scroll(false, false);
        assert_eq!(item.state.selected(), Some(1));

        item.handle_scroll(false, false);
        assert_eq!(item.state.selected(), Some(2));

        item.handle_scroll(false, false);
        assert_eq!(item.state.selected(), Some(2));
        // previous
        item.handle_scroll(true, false);
        assert_eq!(item.state.selected(), Some(1));
        // page down
        item.handle_scroll(false, true);
        assert_eq!(item.state.selected(), Some(2));
        // page up
        item.handle_scroll(true, true);
        assert_eq!(item.state.selected(), Some(0));
    }
}
