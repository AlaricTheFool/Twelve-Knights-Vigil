use crate::prelude::*;

pub fn respawn_message_received(reset_messages: Query<&Reset>) -> bool {
    !reset_messages.is_empty()
}
