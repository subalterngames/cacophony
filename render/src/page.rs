use crate::PagePosition;

/// A page of elements in a scrollable context.
pub(crate) struct Page {
    /// The indices of the visible elements.
    pub visible: Vec<usize>,
    /// A description of the position of this page.
    pub position: PagePosition,
}

impl Page {
    pub(crate) fn new(selected: &Option<usize>, elements: &[u32], height: u32) -> Self {
        // Visible elements.
        let mut visible = vec![];
        // The current height of the page.
        let mut page_h = 0;
        // If true, we found the page.
        let mut this_page = false;
        for (i, element) in elements.iter().enumerate() {
            // There is room for this element. Add it.
            if page_h + *element <= height {
                visible.push(i);
                // Increment.
                page_h += *element;
            } else {
                // It's this page. Stop here.
                if this_page {
                    break;
                }
                // New page.
                visible.clear();
                visible.push(i);
                page_h = *element;
            }
            // This is the page!
            if let Some(selected) = selected {
                if *selected == i {
                    this_page = true;
                }
            } else {
                this_page = true;
            }
        }
        // I guess there is no page??
        if !this_page {
            visible.clear();
        }
        // Are there more elements after the last one?
        let after = match visible.iter().max() {
            Some(max) => *max < elements.len() - 1,
            None => false,
        };
        // What about before?
        let before = match visible.iter().min() {
            Some(min) => *min > 0,
            None => false,
        };
        // Infer the relative position.
        let position = match (after, before) {
            (true, true) => PagePosition::Mid,
            (true, false) => PagePosition::First,
            (false, true) => PagePosition::Last,
            (false, false) => PagePosition::Only,
        };
        Self { visible, position }
    }
}
