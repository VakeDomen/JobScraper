use teloxide::{Bot, types::Message};

use crate::helpers::data_helper::{SUBSCRIBERS, save_subscribers};

pub fn unsubscribe(
    _: &Bot,
    message: Message
) -> String {
    let reposnse_msg;
    let should_save = {
        let mut subs = SUBSCRIBERS.lock().unwrap();
        let index_option = subs.iter().position(|&r| r == message.chat.id.0);
        let changes = if let Some(index) = index_option {
            subs.remove(index);
            reposnse_msg = "Unsubscribed for jobs".to_string();
            true
        } else {
            reposnse_msg = "Already not subscribed for jobs".to_string();
            false
        };
        changes
    };
    if should_save {
        save_subscribers();
    }
    reposnse_msg
}
