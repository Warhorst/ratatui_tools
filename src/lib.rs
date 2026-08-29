use std::{sync::mpsc::{Receiver, Sender, channel}, thread};

use ratatui::crossterm::event;
use ratatui_core::{layout::Rect, terminal::Frame};
use ratatui_crossterm::crossterm::event::Event;

pub mod cells;
pub mod text_input;
pub mod tilemap;

/// Wrapper around ratatui which utilizes the [`Component`] pattern
/// to build TUIs. The Framework manages an internal MPSC channel which allows
/// other threads to send messages to the TUI, which manipulates its state.
pub struct Framework<S, F, M> {
    pub state: S,
    pub focus: F,
    sender: Sender<FrameworkEvent<M>>,
    receiver: Receiver<FrameworkEvent<M>>,
}

impl<S, F, M> Framework<S, F, M> where M: 'static + Send {
    /// Create a new [`Framework`].
    /// - `initial_state`: The initial state of the framework
    /// - `initial_focus`: The initial focus of the framework
    pub fn new(
        initial_state: S,
        initial_focus: F,
    ) -> Self {
        let (sender, receiver) = channel();

        Framework {
            state: initial_state,
            focus: initial_focus,
            sender,
            receiver,
        }
    }

    /// Create a new sender which can be used by external threads to send messages
    /// to the framework.
    pub fn message_sender(&self) -> MessageSender<M> {
        MessageSender::new(self.sender.clone())
    }

    /// Run the framework, which renders the UI and starts the event / message handling.
    ///
    /// - `core_component`: The [`Component`] which contains all other [`Components`]. This is the entrypoint into rendering the TUI and handling events.
    /// - `message_handler`: Closure which processes messages and modifies the state of the app.
    /// - `is_exit`: Closure which checks if a received input event indicates a shutdown of the app. If this returns always false, the app must be closed by other means.
    pub fn run(
        mut self,
        mut core_component: impl Component<S, F, M>,
        message_handler: impl Fn(M, &mut Self),
        is_exit: impl Fn(&Event) -> bool
    ) -> std::io::Result<()> {
        // Start the input handler thread
        let sender = self.sender.clone();
        thread::spawn(move || {
            loop {
                let event = event::read().expect("Input event should be readable");
                sender.send(FrameworkEvent::RatatuiEvent(event)).expect("Failed to send framework event");
            }
        });
    
        ratatui::run(|terminal| {
            loop {
                terminal.draw(|frame| {
                    core_component.render(frame, frame.area(), &self.state, &self.focus);
                })?;

                match self
                    .receiver
                    .recv()
                    .expect("Framework event should be receivable")
                {
                    FrameworkEvent::RatatuiEvent(event) => {
                        if is_exit(&event) {
                            break
                        } else if let Some(message) =
                            core_component.handle_event(event, &self.state, &mut self.focus)
                        {
                            message_handler(message, &mut self)
                        }
                    }
                    FrameworkEvent::AppMessage(message) => {
                        message_handler(message, &mut self)
                    }
                }
            }

            Ok(())
        })
    }
}

/// An event sent to the [`Framework`].
enum FrameworkEvent<M> {
    /// An input event from ratatui
    RatatuiEvent(Event),
    /// A message which causes a state change
    AppMessage(M),
}

/// Allows to send messages to a [`Framework`].
#[derive(Clone)]
pub struct MessageSender<M> {
    sender: Sender<FrameworkEvent<M>>,
}

impl<M> MessageSender<M> {
    fn new(sender: Sender<FrameworkEvent<M>>) -> Self {
        MessageSender { sender }
    }

    /// Send the given messsage to the [`Framework`] which created this [`MessageSender`].
    pub fn send(
        &self,
        message: M,
    ) {
        self.sender
            .send(FrameworkEvent::AppMessage(message))
            .expect("Send of message failed");
    }
}

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
