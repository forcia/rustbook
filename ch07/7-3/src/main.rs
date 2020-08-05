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
    start_stop_button_state: button::State,
    reset_button_state: button::State,
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
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
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
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    /// This method returns widgets to show them in Window.
    fn view(&mut self) -> Element<Self::Message> {
        // init widgets
        let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

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
