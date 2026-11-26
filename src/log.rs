use std::fmt;

use tracing::field::{Field, Visit};
use tracing::span;
use tracing::{Event, Level, Metadata, Subscriber};

pub fn to_stderr(most_detailed: Level) {
    let _ = tracing::subscriber::set_global_default(Stderr { most_detailed });
}

struct Stderr {
    most_detailed: Level,
}

impl Subscriber for Stderr {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        *metadata.level() <= self.most_detailed
    }

    fn new_span(&self, _: &span::Attributes<'_>) -> span::Id {
        span::Id::from_u64(1)
    }

    fn record(&self, _: &span::Id, _: &span::Record<'_>) {}

    fn record_follows_from(&self, _: &span::Id, _: &span::Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut message = Message::default();
        event.record(&mut message);
        eprintln!("{:<5} {}", event.metadata().level(), message.0);
    }

    fn enter(&self, _: &span::Id) {}

    fn exit(&self, _: &span::Id) {}
}

#[derive(Default)]
struct Message(String);

impl Visit for Message {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        if field.name() == "message" {
            self.0.push_str(&format!("{value:?}"));
        } else {
            self.0.push_str(&format!("{}={value:?}", field.name()));
        }
    }
}
