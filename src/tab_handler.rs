use crate::println;

struct Tab {
    name: &'static str,
    handler: fn(),
    id: usize,
}

pub struct TabHandler {
    tabs: [Tab; 2],
    currentTab: usize,
}

impl TabHandler {
    pub fn new() -> Self {
        Self {
            tabs: [
                Tab {
                    name: "Tab 1",
                    handler: || println!("Tab 1"),
                    id: 0,
                },
                Tab {
                    name: "Tab 2",
                    handler: || println!("Tab 2"),
                    id: 1,
                },
            ],
            currentTab: 0,
        }
    }

    pub fn switch_tab(&mut self) {
        self.currentTab = (self.currentTab + 1) % self.tabs.len();
        println!("Switched to {}", self.tabs[self.currentTab].name);
    }
    
    pub fn handle_tab(&self) {
        (self.tabs[self.currentTab].handler)();
    }
}