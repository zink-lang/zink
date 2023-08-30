//! Event implementation

/// Zink event interface
pub trait Event {
    const NAME: &'static [u8];

    /// Returns the first topic.
    fn topic_0(&self) -> Option<[u8; 32]> {
        None
    }

    /// Returns the second topic.
    fn topic_1(&self) -> Option<[u8; 32]> {
        None
    }

    /// Returns the third topic.
    fn topic_2(&self) -> Option<[u8; 32]> {
        None
    }

    /// Returns the fourth topic.
    fn topic_3(&self) -> Option<[u8; 32]> {
        None
    }

    /// Returns the event topics.
    fn topics(&self) -> [Option<[u8; 32]>; 4] {
        [
            self.topic_0(),
            self.topic_1(),
            self.topic_2(),
            self.topic_3(),
        ]
    }

    /// Emit the event.
    fn emit(&self);
}
