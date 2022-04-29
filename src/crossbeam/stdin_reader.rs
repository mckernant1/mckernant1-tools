use std::thread;
use std::thread::JoinHandle;
use crossbeam::channel::Sender;
use log::debug;

pub fn read_stdin(
    stdin_send: Sender<Option<String>>
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let bytes = std::io::stdin().read_line(&mut buf).unwrap();
            if bytes == 0 {
                break;
            }
            stdin_send.send(Some(buf)).unwrap();
        }
        debug!("No More input");
        stdin_send.send(None).unwrap();
    })

}
