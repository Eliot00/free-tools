use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription};
use notify_rust::Notification;

use std::fmt;
use std::time::{Duration, Instant};

const WORK_DURATION: Duration = Duration::from_secs(1500);
const BREAK_DURATION: Duration = Duration::from_secs(300);
const INTERVAL: Duration = Duration::from_secs(10);

pub fn main() -> iced::Result {
    TomatoClock::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (400, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct TomatoClock {
    duration: Duration,
    state: State,
    clock_type: ClockType,
}

impl TomatoClock {
    fn next(&mut self) {
        self.duration = match self.clock_type {
            ClockType::Work => {
                self.clock_type = ClockType::WorkInterval;
                INTERVAL
            }
            ClockType::Break => {
                self.clock_type = ClockType::BreakInterval;
                INTERVAL
            }
            ClockType::WorkInterval => {
                self.clock_type = ClockType::Break;
                BREAK_DURATION
            }
            ClockType::BreakInterval => {
                self.clock_type = ClockType::Work;
                WORK_DURATION
            }
        };
    }
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Clone)]
enum ClockType {
    Work,
    Break,
    WorkInterval,
    BreakInterval,
}

impl fmt::Display for ClockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Self::Work => "Working",
            Self::Break => "Breaking",
            _ => "Interval",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
}

impl Application for TomatoClock {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TomatoClock, Command<Message>) {
        (
            TomatoClock {
                duration: WORK_DURATION,
                state: State::Idle,
                clock_type: ClockType::Work,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TomatoClock")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    let duration = now - *last_tick;
                    if self.duration <= duration {
                        let last_clock_type = self.clock_type.clone();
                        self.next();
                        Notification::new()
                            .summary("TomatoClock")
                            .body(format!("Your [{}] timer is done!", last_clock_type).as_str())
                            .icon("firefox")
                            .show()
                            .ok();
                        return Command::none();
                    }
                    self.duration -= now - *last_tick;
                    *last_tick = now;
                }
            }
            Message::Reset => {
                self.clock_type = ClockType::Work;
                self.duration = WORK_DURATION;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => time::every(Duration::from_millis(10)).map(Message::Tick),
        }
    }

    fn view(&self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(40);

        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(Length::Units(80))
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Toggle)
        };

        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);

        let controls = row![toggle_button, reset_button].spacing(20);

        let status = text(&self.clock_type);

        let content = column![duration, controls, status]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
