use std::{sync::mpsc, thread};

use tokio::{join, select};

/*
 *  Channels
 */
#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let mut children: Vec<_> = Vec::new();

    for id in 0..=9 {
        let tx = tx.clone();

        // each thread will send on the channel
        let child = thread::spawn(move || {
            tx.send(id).unwrap();
            println!("finished sending: {id}");
        });

        children.push(child);
    }

    let mut ids = vec![];

    for _ in 0..=9 {
        // this recv will block the execution
        ids.push(rx.recv().unwrap());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);

    /*
     *   This is not ergonomic because we need to block the thread
     *   to await the values from `rx`. And we are doing this
     *   inside a for loop, it is hard to reuse this code.
     *
     *   The following is more ergonomic due to Futures sugar.
     */

    /*
     *  Now using tokio
     */

    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(32);

    // the handles need to be await'ed
    let sender_handle = tokio::spawn(async move {
        for id in 0..=10 {
            if let Err(e) = tx.send(id).await {
                println!("error {:?}", e);
                return;
            } else {
                println!("sending {id}");
            }
        }
    });

    let receiver_handle = tokio::spawn(async move {
        while let Some(v) = rx.recv().await {
            println!("received id {v}");
        }
    });

    sender_handle.await.unwrap();
    receiver_handle.await.unwrap();

    /*
     *   We generally use channels with events
     */

    #[derive(Debug)]
    enum Event {
        Chocolate,
        Grape,
        Banana,
    }

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(32);

    let daemon = tokio::spawn(async move {
        loop {
            println!("infinite loop?");
            if let Some(e) = rx.recv().await {
                match e {
                    Event::Chocolate => println!("chocolate"),
                    Event::Banana => println!("banana"),
                    Event::Grape => println!("grape"),
                }
            }
        }
    });

    // the handles need to be await'ed
    tokio::spawn(async move {
        tx.send(Event::Chocolate).await.unwrap();
        tx.send(Event::Banana).await.unwrap();
        tx.send(Event::Grape).await.unwrap();
    })
    .await
    .unwrap();

    daemon.await.unwrap();
}
