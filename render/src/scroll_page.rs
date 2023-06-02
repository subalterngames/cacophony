/// Converts a list of elements into a viewable page.
///
/// - `selected` The index of the current selection.
/// - `elements` A list of elements. Each value is the *height* of the element in grid units.
/// - `height` The height of the viewable area.
pub fn get_page(selected: &Option<usize>, elements: &[u32], height: u32) -> Vec<usize> {
    // Generate a page of tracks.
    let mut track_page: Vec<usize> = vec![];
    let mut page_h = 0;
    let mut this_page = false;
    for (i, element) in elements.iter().enumerate() {
        // There is room for this track. Add it.
        if page_h + *element <= height {
            track_page.push(i);
            // Increment.
            page_h += *element;
        } else {
            // It's this page. Stop here.
            if this_page {
                break;
            }
            // New page.
            track_page.clear();
            track_page.push(i);
            page_h = *element;
        }
        // This is the page!
        if let Some(selected) = selected {
            if *selected == i {
                this_page = true;
            }
        }
    }
    // We couldn't find the any selected track.
    if !this_page {
        track_page.clear();
    }
    track_page
}
