use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, channel};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: Sender<Command>
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        // create a new channel, get response channel in/out
        let (tx, rx) = channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: tx, // pass the sender to the server
            }).unwrap();
        rx.recv().unwrap() // check the server response on the reciever
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (tx, rx) = channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel: tx,
            }).unwrap();
        rx.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (commands_tx, commands_rx) = channel();
    std::thread::spawn(move || server(commands_rx));
    TicketStoreClient {sender: commands_tx}
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
