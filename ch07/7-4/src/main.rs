use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Text,
};

// font settings
//
// include_bytes macro will attach the specified file contents to executable binary.
// We can use it as binary array.
const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}

/// Application's main structure
struct GUI {
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

/// The state is for start/stop button.
/// The button need to toggle the functionalities.
pub enum TickState {
    Stopped,
    Ticking,
}

/// Message is sent to the method update().
#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
}

/// When you use iced crate, you need to choose whether Application or Sandbox.
///
/// Application has more functionalities than Sandbox.
/// One of advanced Application's feature is Subscription.
/// It can execute asyncronous functions.
///
/// If you would like to redraw window contents at regular intervals, such like game or animation, you need to use subscription to produce original redraw events.
///
/// And, if you need to handle initial flags and options, need to choose Application.
///
/// If you don't need rich UI, just need simpler UI without asyncronous executions, you should choose Sandbox.
///
/// In this case, we will use Application because we would like to redraw window contents at regular intervals.
impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                tick_state: TickState::Stopped,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    /// Sets the window title.
    fn title(&self) -> String {
        String::from("STOPWATCH")
    }

    /// This method receives Message as event.
    /// And, it changes the state of application by the message.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Reset => {}
        }
        Command::none()
    }

    /// This method returns widgets to show them in Window.
    fn view(&mut self) -> Element<Self::Message> {
        // prepare duration text
        let duration_text = "00:00:00.00";

        // prepare start/stop text
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
                TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };

        // prepare start/stop message on button press
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        // init widgets
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message);
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Reset);

        // prepare column
        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}
