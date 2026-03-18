use crate::Component;
use ratatui_core::{
    layout::{Position, Rect},
    terminal::Frame,
};
use ratatui_crossterm::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui_widgets::paragraph::Paragraph;

/// [Component] which provides a single line text input.
///
/// It provides the following default key mappings:
/// * `Backspace`: Delete the character before the cursor
/// * `Left`: Move the cursor to the left
/// * `Right`: Move the cursor to the right
/// * `Home`: Move the cursor to the start of the input
/// * `End`: Move the cursor to the end of the input
#[allow(clippy::type_complexity)]
pub struct TextInput<F, M> {
    cursor_pos: u16,
    is_focused: Box<dyn Fn(&F) -> bool + 'static>,
    handle_input: Box<dyn Fn(&Event) -> Option<M> + 'static>,
    text_message_map: Box<dyn Fn(String) -> M + 'static>,
}

impl<F, M> TextInput<F, M> {
    /// Create a new [TextInput] from the provided parameters.
    /// * `is_focused`: Determines based on the focus type `F` if this [TextInput] is focused
    /// * `handle_input`: A closure that gets called when an [Event] is received by the [TextInput] and
    ///   Before the default event handling. This allows to send a message `M` if some special input is received.
    /// * `text_message_map`: If the text was modified based on the received [Event], this closure is uses to map
    ///   The new [String] to a message `M`.
    pub fn new(
        is_focused: impl Fn(&F) -> bool + 'static,
        handle_input: impl Fn(&Event) -> Option<M> + 'static,
        text_message_map: impl Fn(String) -> M + 'static,
    ) -> Self {
        TextInput {
            cursor_pos: 0,
            is_focused: Box::new(is_focused),
            handle_input: Box::new(handle_input),
            text_message_map: Box::new(text_message_map),
        }
    }
}

impl<F, M> Component<String, F, M> for TextInput<F, M> {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        input: &String,
        focus: &F,
    ) {
        if self.cursor_pos > input.chars().count() as u16 {
            // If the text suddenly is smaller than the last cursor position, reset the cursor
            self.cursor_pos = 0;
        }

        let input = Paragraph::new(input.as_str());
        frame.render_widget(input, area);

        if (self.is_focused)(focus) {
            frame.set_cursor_position(Position::new(area.x + self.cursor_pos, area.y));
        }
    }

    fn handle_event(
        &mut self,
        event: Event,
        input: &String,
        focus: &mut F,
    ) -> Option<M> {
        if !(self.is_focused)(focus) {
            // If the text input is not focused, do nothing
            return None
        }
        
        // If some special character was pressed which produces a message, don't forward the mesasge to the
        // handler and instead return the message prematurely.
        let msg = (self.handle_input)(&event);

        if msg.is_some() {
            return msg;
        }

        if let Event::Key(key_event) = event
            && key_event.kind == KeyEventKind::Press
        {
            match key_event.code {
                KeyCode::Char(c) => {
                    let mut new_text = "".to_string();
                    let new_cursor_pos = self.cursor_pos + 1;

                    // Add all existing characters until the cursor position
                    input
                        .chars()
                        .take(self.cursor_pos as usize)
                        .for_each(|c| new_text.push(c));
                    // Add the new char
                    new_text.push(c);
                    // Add all remaining characters from the existing text
                    input
                        .chars()
                        .skip(self.cursor_pos as usize)
                        .for_each(|c| new_text.push(c));
                    self.cursor_pos = new_cursor_pos;

                    return Some((self.text_message_map)(new_text));
                }
                KeyCode::Backspace if self.cursor_pos > 0 => {
                    let mut new_text = "".to_string();
                    let new_cursor_pos = self.cursor_pos - 1;

                    // Add all existing characters until the new cursor position, which
                    // will skip a character (the deleted one), as it is now smaller
                    input
                        .chars()
                        .take(new_cursor_pos as usize)
                        .for_each(|c| new_text.push(c));
                    // Add the remaining characters from the existing text
                    input
                        .chars()
                        .skip(self.cursor_pos as usize)
                        .for_each(|c| new_text.push(c));
                    self.cursor_pos = new_cursor_pos;

                    return Some((self.text_message_map)(new_text));
                }
                KeyCode::Left if self.cursor_pos > 0 => {
                    self.cursor_pos -= 1;
                }
                KeyCode::Right if self.cursor_pos < input.chars().count() as u16 => {
                    self.cursor_pos += 1;
                }
                KeyCode::Home => {
                    self.cursor_pos = 0;
                }
                KeyCode::End => {
                    self.cursor_pos = input.chars().count() as u16;
                }
                _ => {}
            }
        }

        None
    }
}
