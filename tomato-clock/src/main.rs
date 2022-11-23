use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription};
use notify_rust::Notification;

use std::time::{Duration, Instant};

const WORK_DURATION: Duration = Duration::from_secs(1500);
const BREAK_DURATION: Duration = Duration::from_secs(300);

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

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

enum ClockType {
    Work,
    Break,
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
                        self.duration = match self.clock_type {
                            ClockType::Work => {
                                self.clock_type = ClockType::Break;
                                BREAK_DURATION
                            }
                            ClockType::Break => {
                                self.clock_type = ClockType::Work;
                                WORK_DURATION
                            }
                        };
                        Notification::new()
                            .summary("TomatoClock")
                            .body("Your timer is done!")
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

        let status = text(match self.clock_type {
            ClockType::Work => "Working",
            ClockType::Break => "Breaking",
        });

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
