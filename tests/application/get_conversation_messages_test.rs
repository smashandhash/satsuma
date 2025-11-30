#[cfg(test)]
mod tests {
    use satsuma::{
        application::load_chat_container_messages::{
            LoadChatContainerMessagesUseCase,
            LoadChatContainerMessagesUseCaseError,
        },
        domain::{
            chat_container::{
                ChatContainer,
                ChatContainerContext
            },
            chat_session::ChatSession
        },
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;

    #[test]
    fn successfully_load_messages() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()],
            Vec::new());
        let repository = ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: Some(chat_container),
        };
    }
    /*
    use satsuma::{
        application::load_chat_container_messages::{
            LoadChatContainerMessagesUseCase,
            LoadChatContainerMessagesUseCaseError,
        },
        domain::message::{
            Message,
            MessageKind
        }
    };
    use crate::helper::message_repository_stub::MessageRepositoryStub;
    use chrono::Utc;
    use rstest::rstest;

    #[rstest]
    #[case("conversation between two users", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Hello, Bob".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub2134".to_string())), Message::new("id2".to_string(), "npub2134".to_string(), "Hello, Alice".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub1234".to_string()))], "npub1234", "npub2134", 2)]
    #[case("empty conversation", Vec::new(), "npub1234", "npub2134", 0)]
    #[case("self conversation", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Note to myself".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub1234".to_string()))], "npub1234", "npub1234", 1)]
    #[case("conversation of different ID", vec![Message::new("id1".to_string(), "npub2134".to_string(), "Hello!".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub3124".to_string())), Message::new("id2".to_string(), "npub3124".to_string(), "Hi!".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub2134".to_string()))], "npub1234", "npub2134", 1)]
    fn get_conversation_messages(
        #[case] _label: &str,
        #[case] messages: Vec<Message>,
        #[case] sender_public_key: &str,
        #[case] recipient_public_key: &str,
        #[case] conversation_length: usize
        ) {
        let repository = MessageRepositoryStub::new(None, messages.clone());
        let use_case = GetConversationMessagesUseCase { repository };

        let conversation = use_case.execute(sender_public_key.to_string(), recipient_public_key.to_string());

        assert_eq!(conversation.len(), conversation_length);
    }
    */
}
