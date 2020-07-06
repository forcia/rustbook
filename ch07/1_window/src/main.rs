use iced::{executor, Application, Command, Element, Settings, Text};

fn main() {
    GUI::run(Settings::default());
}

/// Application's main structure
struct GUI;

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
        (GUI, Command::none())
    }

    /// Sets the window title.
    fn title(&self) -> String {
        String::from("DEMO")
    }

    /// This method receives Message as event.
    /// And, it changes the state of application by the message.
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    /// This method returns widgets to show them in Window.
    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, World!").into()
    }
}
