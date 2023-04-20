mod html;
use networking::get_html_content;
use tracing::info;

pub struct TUI {
    name: String,
}

impl Default for TUI {
    fn default() -> Self {
        Self::new()
    }
}

impl TUI {
    pub fn new() -> Self {
        Self {
            name: "TUI".to_string(),
        }
    }

    pub async fn run(&mut self) {
        info!("{} is running", self.name);

        // retrive the data from network
        let content = get_html_content("https://www.google.com").await;

        let links = html::parse_html(content.unwrap().as_str());
    }
}
