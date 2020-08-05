use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
use iced_futures::{self, futures};
use std::time::{Duration, Instant};

// font settings
//
// include_bytes macro will attach the specified file contents to executable binary.
// We can use it as binary array.
const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}

/// Application's main structure
struct GUI {
    last_update: Instant,
    total_duration: Duration,
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
///
/// Some of messages are produced by pressing button.
/// The other (Update message) is produced by the subscription.
#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Update,
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
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                last_update: Instant::now(),
                total_duration: Duration::default(),
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
    ///
    /// There are some kinds of messages in iced framework.
    ///
    /// Some messages are for application events, window manager events and so on.
    /// If the window is resized or cursor is clicked or other system events are happened, this method receives this kind of message.
    ///
    /// Other messages are for widget events.
    /// When buttons are pressed or sliders are moved or other widgets are changed, this method receives messages.
    ///
    /// And, other messages are for subscription.
    /// The subscription is executed asynchronously by the method subscription().
    ///
    /// In this application, this method handles messages from widgets and subscription.
    /// And, it changes the state of application by the message.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration += Instant::now() - self.last_update;
            }
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::default();
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
        }
        Command::none()
    }

    /// this subscription method will be called once at app's starting.
    /// Then, Subscription will be produced.
    ///
    /// The subscription will continuously produce the event.
    /// The event will be handled in the method update().
    fn subscription(&self) -> Subscription<Message> {
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }

    /// This method returns widgets to show them in Window.
    ///
    /// Regarding Sandbox, this method will not be called so frequently.
    /// At the application startup, and at application event or window event, it will be called.
    ///
    /// On the other hand, regarding Application, this method will be called by subscription events other than events above.
    fn view(&mut self) -> Element<Self::Message> {
        // prepare duration text
        let seconds = self.total_duration.as_secs();
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10,
        );

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

/// This recipe of subscription will produce events repeatly at specified intervals.
/// The interval is determined by the member variable "duration".
pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration: duration }
    }
}

/// This is a recipe of subscription.
///
impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    /// The subscription will produce this Output type.
    type Output = Instant;

    /// Hashes this recipe.
    /// Uses this hash to identify each subscriptions.
    /// You can generate hash from any member variables.
    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    /// This method is in charge of executing this recipe and producing the stream of events of its subscription.
    /// The stream outputs the Output type which you defined.
    /// In this case, the Output type is Instant.
    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}
