use ratatui_core::{layout::Rect, terminal::Frame};
use ratatui_crossterm::crossterm::event::Event;

pub mod cells;
pub mod text_input;
pub mod tilemap;

/// A [Component] is a struct which combines the rendering of some UI component and the handling
/// of events. The event handling emmits messages, which are intended for an [ELM](https://guide.elm-lang.org/architecture/) like architecture.
///
/// The component has the following generic parameters:
/// - `S`: The type of state which gets rendered by the [Component].
/// - `F`: The type of the app focus, which tells which [Component] is currently focused and can be modified when handling input.
/// - `M`: The type of message which might get returned by this [Component] uppon handling events.
///
/// A [Component] should itself only hold state of its widgets and other components to ensure a clean ELM architecture.
///
/// A [Component] is designed to be created once. It should not be recreated every time like a ratatui widget.
///
/// The focus `F` should be the same for every component in the app. This allows to change focus to any other component or widget
/// no matter where you currently are. The state `S` and the message `M` however can be set for the current context. For example, a settings
/// panel in an app is only interested in the settings in the app and might only emmit messages that cause changes in the settings.
pub trait Component<S, F, M> {
    /// Render this component.
    /// * `frame`: The [Frame] on which this [Component] should be drawn.
    /// * `area`: The render area assigned to this [Component] by its parent.
    /// * `state`: The state to render.
    /// * `focus` The current focus of the app. Might be used to draw special highlighting.
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &S,
        focus: &F,
    );

    /// Handle an [Event] and maybe emmit a message `M` as a response. Takes a mutable reference to the
    /// [Component] to for example change widget state based on input which does not a message.
    /// * `event`: The [Event] to handle.
    /// * `state`: The current state which this [Component] handles. It gets not mutated here, but might be required to create messages.
    /// * `focus`: The current focus of the app. Can be used to check if the input should be handled at all or to mutate it based on the input.
    fn handle_event(
        &mut self,
        event: Event,
        state: &S,
        focus: &mut F,
    ) -> Option<M> {
        let _ = event;
        let _ = state;
        let _ = focus;
        None
    }
}
