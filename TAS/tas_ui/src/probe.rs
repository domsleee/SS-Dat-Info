//! Named widget rects for the headless frame tests: panels tag the widgets a
//! test clicks, by a stable name, and the test reads the rect the last frame
//! drew. Compiled out of the app.

use eframe::egui;

#[cfg(test)]
thread_local! {
    static RECTS: std::cell::RefCell<std::collections::HashMap<String, egui::Rect>> =
        Default::default();
}

/// Tag `response` as `name`.
pub(crate) fn tag(response: &egui::Response, name: &str) {
    tag_with(response, || name.to_string());
}

/// Tag `response` with a name built only under test.
pub(crate) fn tag_with(response: &egui::Response, name: impl FnOnce() -> String) {
    #[cfg(test)]
    RECTS.with(|r| r.borrow_mut().insert(name(), response.rect));
    #[cfg(not(test))]
    let _ = (response, name);
}

/// Forget every rect: called before each test frame.
#[cfg(test)]
pub(crate) fn clear() {
    RECTS.with(|r| r.borrow_mut().clear());
}

/// The rect `name` was drawn at in the last frame.
#[cfg(test)]
pub(crate) fn rect(name: &str) -> Option<egui::Rect> {
    RECTS.with(|r| r.borrow().get(name).copied())
}
