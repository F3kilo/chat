use chrono::{DateTime, Utc};
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Chat {
    rooms: Arc<DashMap<String, Room>>,
}

impl Chat {
    pub fn fetch(&self, room_id: &str) -> Option<Vec<String>> {
        let room = self.rooms.get(room_id)?;
        let messages = room.messages().map(|m| m.to_string()).collect();
        Some(messages)
    }

    pub fn create_room(&self, room_id: String) -> Option<String> {
        let room_entry = self.rooms.entry(room_id.clone());
        match room_entry {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                let room = Room::new(&room_id);
                v.insert(room);
                Some(room_id)
            }
        }
    }

    pub fn append(&self, room_id: &str, msg: String) -> Option<DateTime<Utc>> {
        let mut room = self.rooms.get_mut(room_id)?;
        room.append(msg)
    }
}

pub struct Room {
    msgs: Vec<Message>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        let mut new = Self {
            msgs: Default::default(),
        };

        let init_msg = format!("Created with name {}\r\n", name);
        new.append(init_msg);
        new
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.msgs.iter()
    }

    pub fn append(&mut self, msg: String) -> Option<DateTime<Utc>> {
        let dt = Utc::now();
        self.msgs.push(Message(dt, msg));
        Some(dt)
    }
}

pub struct Message(DateTime<Utc>, String);

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Chat;

    #[test]
    fn fetch_after_append() {
        let chat = Chat::default();

        let room_id_1 = "room_1".into();
        let room_id_2 = "room_2".into();

        let room1 = chat.create_room(room_id_1).unwrap();
        let room2 = chat.create_room(room_id_2).unwrap();

        let msg1 = String::from("Some message");
        let msg2 = String::from("Other message");

        chat.append(&room1, msg1).unwrap();
        chat.append(&room1, msg2.clone()).unwrap();

        chat.append(&room2, msg2).unwrap();

        assert_eq!(chat.fetch(&room1).unwrap().len(), 3);
        assert_eq!(chat.fetch(&room2).unwrap().len(), 2);
    }
}
