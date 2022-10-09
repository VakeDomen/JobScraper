use teloxide::{Bot, types::Message};

use crate::helpers::data_helper::{SUBSCRIBERS, save_subscribers};

pub fn subscribe(
    _: &Bot,
    message: Message,
) -> String {
    let reposnse_msg;
    let should_save = {
        let mut subs = SUBSCRIBERS.lock().unwrap();
        let changes = if !subs.contains(&message.chat.id.0) {
            subs.push(message.chat.id.0);
            reposnse_msg = "Subscribed for jobs".to_string();
            true
        } else {
            reposnse_msg = "Already subscribed for jobs".to_string();
            false
        };
        changes
    };
    if should_save {
        save_subscribers();
    }
    reposnse_msg
}
