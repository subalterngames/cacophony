use crate::panel::*;
use crate::Popup;
use input::InputEvent;
use text::Tooltips;

const NUM_LINKS: usize = 3;
const LABEL_COLOR: ColorKey = ColorKey::Value;

/// Open a link in a browser.
pub(crate) struct LinksPanel {
    /// The panel background.
    panel: Panel,
    /// The labels.
    labels: [Label; NUM_LINKS],
    /// The popup.
    pub popup: Popup,
}

impl LinksPanel {
    pub fn new(config: &Ini, text: &Text, input: &Input) -> Self {
        // Get each label. The x coordinate is right now at 0.
        let mut y = MAIN_MENU_HEIGHT + 1;
        let mut tooltips = Tooltips::default();
        let mut website = Self::get_label(
            &mut tooltips,
            &mut y,
            "LINKS_PANEL_WEBSITE",
            InputEvent::WebsiteUrl,
            text,
            input,
        );
        let mut discord = Self::get_label(
            &mut tooltips,
            &mut y,
            "LINKS_PANEL_DISCORD",
            InputEvent::DiscordUrl,
            text,
            input,
        );
        let mut github = Self::get_label(
            &mut tooltips,
            &mut y,
            "LINKS_PANEL_GITHUB",
            InputEvent::GitHubUrl,
            text,
            input,
        );
        // Get the maximum width of the labels.
        let max_w = [
            &website.text,
            &discord.text,
            &github.text,
            text.get_ref("TITLE_LINKS"),
        ]
        .iter()
        .map(|s| s.chars().count() as u32)
        .max()
        .unwrap();
        // Get the panel width.
        let w = max_w + 4;
        // Get the window width.
        let window_grid_size = get_window_grid_size(config);
        // Get the panel x and y coordinates and height.
        let x = window_grid_size[0] / 2 - w / 2;
        let h = y - MAIN_MENU_HEIGHT;
        y = MAIN_MENU_HEIGHT;
        // Set the x coordinates.
        let label_x = x + 1;
        website.position[0] = label_x;
        discord.position[0] = label_x;
        github.position[0] = label_x;
        // Define the panel.
        let panel = Panel::new(PanelType::Links, [x, y], [w, h], text);
        let labels = [website, discord, github];
        let popup = Popup::new(PanelType::Links);
        Self {
            panel,
            labels,
            popup,
        }
    }

    /// Returns a label and moves the y coordinate.
    fn get_label(
        tooltips: &mut Tooltips,
        y: &mut u32,
        key: &str,
        event: InputEvent,
        text: &Text,
        input: &Input,
    ) -> Label {
        let label = Label::new(
            [0, *y],
            tooltips.get_tooltip(key, &[event], input, text).seen,
        );
        *y += 2;
        label
    }
}

impl Drawable for LinksPanel {
    fn update(&self, renderer: &Renderer, _: &State, _: &Conn, _: &Text, _: &PathsState) {
        self.popup.update(renderer);
        self.panel.update(true, renderer);
        self.labels
            .iter()
            .for_each(|label| renderer.text(label, &LABEL_COLOR));
    }
}
