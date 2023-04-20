use crate::cli::Cli;

#[allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
pub enum UI {
    TUI(tui::TUI),
    GUI,
}

/// define the application, contains all the configurations
pub struct App {
    instance: UI,
}

impl App {
    pub fn new() -> Self {
        // we should use config file and merge it with cli
        // TODO: use config file
        // parse the cli
        // TODO: use cli to config
        let cli = Cli::new();

        // if no mode is provided, use the tui mode
        let instance = if let Some(mode) = cli.mode {
            match mode.as_str() {
                "tui" => UI::TUI(tui::TUI::new()),
                "gui" => UI::GUI,
                _ => UI::TUI(tui::TUI::new()),
            }
        } else {
            UI::TUI(tui::TUI::new())
        };

        App { instance }
    }

    pub async fn run(&mut self) {
        // check instance
        match &mut self.instance {
            UI::TUI(tui) => tui.run().await,
            UI::GUI => todo!(),
        }
    }
}
