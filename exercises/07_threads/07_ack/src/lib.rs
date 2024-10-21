use std::sync::mpsc::{Receiver, Sender};
use data::{Ticket, TicketDraft};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { draft: TicketDraft, response_sender: Sender<TicketId> },
    Get { id: TicketId, response_sender: Sender<Option<Ticket>> }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft, response_sender}) => {
                let id = store.add_ticket(draft);
               let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id,
                response_sender
            }) => {
                let ticket = store.get(id);

               let _ = match ticket {
                    Some(t) => response_sender.send(Some(t.clone())),
                    None => response_sender.send(None),
                };
                
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
