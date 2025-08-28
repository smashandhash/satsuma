use satsuma::domain::conversation::Conversation;

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn conversation_init_should_do_nothing() {
        let conversation = Conversation::new(1, vec![101, 202]);
    }
}
