use crate::chat::Chat;
use std::str::Split;

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    chat: Chat,
}

impl RequestHandler {
    pub fn new(chat: Chat) -> Self {
        Self { chat }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next();
        match command {
            "fetch" => self.fetch(request),
            "create" => self.create_room(request),
            "append" => self.append(request),
            _ => "Bad command".into(),
        }
    }

    fn fetch(&self, mut request: Request) -> String {
        let room_id = request.next();
        if room_id.is_empty() {
            return "Select room id".into();
        }

        let messages = match self.chat.fetch(room_id) {
            Some(m) => m,
            None => return "Bad room id".into(),
        };

        const MEAN_MESSAGE_LEN: usize = 256;
        let buf = String::with_capacity(MEAN_MESSAGE_LEN * messages.len());
        messages.into_iter().fold(buf, |mut acc, msg| {
            acc.push_str(&msg);
            acc
        })
    }

    fn create_room(&mut self, mut request: Request) -> String {
        let room_id = request.next();
        if room_id.is_empty() {
            return "Select room id".into();
        }

        match self.chat.create_room(room_id.into()) {
            Some(r) => format!("Room `{}` created", r),
            None => format!("Room `{}` already exists", room_id),
        }
    }

    fn append(&self, mut request: Request) -> String {
        let room_id = request.next();
        if room_id.is_empty() {
            return "Select room id".into();
        }

        let msg = request.next();
        if msg.is_empty() {
            return "Enter some message".into();
        }

        if let Some(dt) = self.chat.append(room_id, msg.into()) {
            format!("Appended at {}", dt)
        } else {
            "Bad room id".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chat, Request, RequestHandler};

    #[test]
    fn append_fetch() {
        let chat = Chat::default();
        let mut handler = RequestHandler::new(chat);

        let room_id = String::from("room_1");
        let req_str = format!("create|||{}", room_id);
        let req = Request::new(&req_str);
        assert_eq!(handler.handle(req), format!("Room `{}` created", room_id));

        let msg = String::from("Some msg\r\n");
        let req_str = format!("append|||{}|||{}", room_id, msg);
        let req = Request::new(&req_str);
        handler.handle(req);

        let msg = String::from("Other msg\r\n");
        let req_str = format!("append|||{}|||{}", room_id, msg);
        let req = Request::new(&req_str);
        handler.handle(req);

        let req_str = format!("fetch|||{}", room_id);
        let req = Request::new(&req_str);
        let fetched = handler.handle(req);

        println!("{}", fetched);
    }
}
