/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?

pub struct Message {
    payload: String,
    response_channel: tokio::sync::mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: tokio::sync::mpsc::Receiver<Message>) {
    loop {
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = tokio::sync::mpsc::channel::<Message>(1);
            let _ = msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                }).await;
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = tokio::sync::mpsc::channel::<Message>(1);
        let (response_sender, mut response_receiver) = tokio::sync::mpsc::channel::<Message>(1);
        let var_name = Message {
                payload: "pong".into(),
                response_channel: response_sender,
            };
        let _ = sender
            .send(var_name).await;

        tokio::spawn(pong(receiver));

        let recv = response_receiver.recv().await;
        let answer = recv.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
