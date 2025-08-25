use satsuma::domain::message::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_does_nothing() {
        let message = Message::new(1, 100, 200, "Hello!");
    }
}
