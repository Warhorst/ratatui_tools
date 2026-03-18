use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::Rect,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem},
};
use ratatui_tools::{Component, text_input::TextInput};

pub fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut state = State::default();
        let mut app = App::default();

        while !state.should_exit {
            terminal.draw(|frame| {
                app.render(frame, frame.area(), &state, &());
            })?;

            let message_opt = app.handle_event(event::read()?, &state, &mut ());

            if let Some(message) = message_opt {
                match message {
                    Message::Quit => state.should_exit = true,
                    Message::UpdateText(text) => state.text = text,
                    Message::Submit => {
                        state.messages.push(state.text.clone());
                        state.text.clear();
                    }
                }
            }
        }

        Ok(())
    })
}

#[derive(Default)]
struct State {
    should_exit: bool,
    text: String,
    messages: Vec<String>,
}

enum Message {
    Quit,
    UpdateText(String),
    Submit,
}

pub struct App {
    text_input: TextInput<(), Message>,
    message_board: MessageBoard,
}

impl Default for App {
    fn default() -> Self {
        App {
            message_board: MessageBoard,
            text_input: TextInput::new(
                |_: &()| true,
                |event| {
                    if let Event::Key(key_event) = event
                        && key_event.kind == KeyEventKind::Press
                        && key_event.code == KeyCode::Enter
                    {
                        Some(Message::Submit)
                    } else {
                        None
                    }
                },
                Message::UpdateText,
            ),
        }
    }
}

impl Component<State, (), Message> for App {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &State,
        _: &(),
    ) {
        let block = Block::bordered()
            .title("Text Input")
            .border_set(border::THICK);

        let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]);
        let [text_input_area, messages_area] = block.inner(area).layout(&layout);

        frame.render_widget(block, area);

        self.text_input
            .render(frame, text_input_area, &state.text, &());
        self.message_board.render(frame, messages_area, state, &());
    }

    fn handle_event(
        &mut self,
        event: Event,
        state: &State,
        _: &mut (),
    ) -> Option<Message> {
        if let Event::Key(key_event) = event
            && key_event.kind == KeyEventKind::Press
        {
            match key_event.code {
                KeyCode::Esc => return Some(Message::Quit),
                _ => return self.text_input.handle_event(event, &state.text, &mut ()),
            }
        }

        None
    }
}

#[derive(Default)]
struct MessageBoard;

impl Component<State, (), Message> for MessageBoard {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &State,
        _focus: &(),
    ) {
        let messages = state
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect::<Vec<_>>();
        let message_block = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(message_block, area);
    }
}
