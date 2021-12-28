use async_trait::async_trait;
use chat_client::asnc::ChatClient;
use std::{io, mem};

#[async_trait]
pub trait State {
    async fn update(&mut self, client: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

#[async_trait]
impl State for Main {
    async fn update(&mut self, _: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Join room
    2) Create room
    Other) Exit"
        );
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => Ok(Box::new(JoinRoom)),
            "2" => Ok(Box::new(CreateRoom)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct Exit;

#[async_trait]
impl State for Exit {
    async fn update(&mut self, _: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}

struct JoinRoom;

#[async_trait]
impl State for JoinRoom {
    async fn update(&mut self, chat: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter room name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        println!("Joining room: {}", name);

        let join_result = chat.fetch(name).await?;
        println!("{}", join_result);

        if join_result == "Bad room id" {
            Ok(Box::new(Main))
        } else {
            Ok(Box::new(InRoom::new(name.into())))
        }
    }
}

struct InRoom(String);

impl InRoom {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[async_trait]
impl State for InRoom {
    async fn update(&mut self, chat: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Send message
    2) Show chat
    3) Leave room
    Other) Exit"
        );

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => Ok(Box::new(SendMessage::new(mem::take(&mut self.0)))),
            "2" => {
                println!("{}", chat.fetch(&self.0).await?);
                Ok(Box::new(InRoom::new(mem::take(&mut self.0))))
            }
            "3" => Ok(Box::new(Main)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct SendMessage(String);

impl SendMessage {
    pub fn new(room: String) -> Self {
        Self(room)
    }
}

#[async_trait]
impl State for SendMessage {
    async fn update(&mut self, chat: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter message:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        chat.append(&self.0, &buf).await?;
        Ok(Box::new(InRoom::new(mem::take(&mut self.0))))
    }
}

struct CreateRoom;

#[async_trait]
impl State for CreateRoom {
    async fn update(&mut self, chat: &mut ChatClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter new room name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        println!("Creating room: {}", name);
        println!("Result: {}", chat.create_room(name).await?);
        Ok(Box::new(InRoom::new(name.into())))
    }
}
