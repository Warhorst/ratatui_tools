use std::{thread, time::Duration};

use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem},
};
use ratatui_tools::{Component, Framework, text_input::TextInput};

const NEWS: [&str; 5] = [
    "Rust <3",
    "ratatui rocks!",
    "The cake was not a lie 🍰",
    "All your base are belong to us",
    "I hope you are doing well.",
];

// This example is an adapted version of the text_input example. It uses the Framework
// wrapper to make the setup easier and provide message sending from multiple threads.
// It adds a "news feed" which shows repeating news that change every 3 seconds. The next
// news are triggered by a background thread which uses the frameworks message sender
// to trigger the change.

pub fn main() -> std::io::Result<()> {
    let framework = Framework::new(State::default(), ());

    let sender = framework.message_sender();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs_f32(3.0));
            sender.send(Message::NextNews);
        }
    });

    framework.run(
        App::default(),
        |message, fw| match message {
            Message::UpdateText(text) => fw.state.text = text,
            Message::Submit => {
                fw.state.messages.push(fw.state.text.clone());
                fw.state.text.clear();
            }
            Message::NextNews => fw.state.next_news(),
        },
        |event| {
            if let Event::Key(key_event) = event
                && key_event.kind == KeyEventKind::Press
                && key_event.code == KeyCode::Esc
            {
                true
            } else {
                false
            }
        },
    )
}

#[derive(Default)]
struct State {
    text: String,
    messages: Vec<String>,
    current_news: usize,
}

impl State {
    fn next_news(&mut self) {
        if self.current_news == NEWS.len() - 1 {
            self.current_news = 0
        } else {
            self.current_news += 1
        }
    }
}

enum Message {
    UpdateText(String),
    Submit,
    NextNews,
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

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [text_input_area, news_area, messages_area] = block.inner(area).layout(&layout);

        frame.render_widget(block, area);
        let news = Line::from(Span::raw(format!(
            "The current news: {}",
            NEWS[state.current_news]
        )));
        frame.render_widget(news, news_area);

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
            return self.text_input.handle_event(event, &state.text, &mut ());
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
