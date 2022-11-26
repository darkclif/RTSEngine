use core::{time, panic};
use std::{net::UdpSocket, thread, time::Duration, cmp::min, sync::{Condvar, Arc, Mutex}, clone};
use futures::{select, FutureExt, pin_mut, executor::block_on};

struct Packet{
    payload: [u8; 1024],
    true_size: usize,
}

impl Packet {
    pub fn from_str(str: &String) -> Packet {
        let mut buffer = [0;1024];
        buffer[..str.len()].copy_from_slice(&str.as_bytes());

        Packet { payload: buffer, true_size: str.len() }
    }
}

struct RingPacketStorage<const N: usize> {
    storage: [Packet; N],
    idx_top: usize,     // out
    idx_bottom: usize,  // in
}

impl<const N: usize> RingPacketStorage<N> {
    fn put_packet(&mut self, packet: Packet){
        if self.idx_bottom == self.idx_top 
        {
            panic!("");
        }

        self.idx_bottom = (self.idx_bottom + 1) % N;
        self.storage[self.idx_bottom] = packet;
    }

    fn take_packet(&mut self) -> &Packet {
        let idx_packet = self.idx_top;
        self.idx_top += 1;
        return &self.storage[idx_packet]
    }

}

pub async fn wait_for_cond(pair: &Arc<(Mutex<bool>, Condvar)>){
    println!("Waiting for conditional.");
    let (run_mutex, cvar) = &**pair;
    let mut run = run_mutex.lock().unwrap();
    while *run {
        run = cvar.wait(run).unwrap();
    }
}

pub async fn recv_udp(socket: &UdpSocket) -> (usize, [u8; 1024]) {
    println!("Waiting for receive.");
    let mut buffer = [0; 1024];
    let (size, src) = socket.recv_from(&mut buffer).unwrap();
    return (size, buffer)
}

pub fn start_udp_server(){
    let (receive_in, receive_out) = std::sync::mpsc::channel();
    let (send_in, send_out) = std::sync::mpsc::channel();

    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let clone_pair = Arc::clone(&pair);

    // UDP Receiver
    let handle_receiver = std::thread::spawn(move || block_on(async {
        let clone_pair_in = clone_pair;

        let socket = UdpSocket::bind("127.0.0.1:7878").unwrap();
        let receiver = Mutex::new(receive_in);
        let mut in_run = true;

        while in_run {
            let signal = wait_for_cond(&clone_pair_in).fuse();
            let packet = recv_udp(&socket).fuse();

            pin_mut!(signal, packet);

            println!("Select started:");
            select! {
                () = signal => {
                    print!("Receiver thread signaled for exit.");
                    in_run = false;
                },
                (size, buffer) = packet => {
                    println!("THREAD: Received: {:#?}", std::str::from_utf8(&buffer[..size]).unwrap());
                    
                    let receiver = receiver.lock().unwrap();
                    receiver.send(Packet{payload: buffer, true_size: size}).unwrap();
                },
            }

            //let mut buffer = [0; 1024];
            //let (size, src) = socket.recv_from(&mut buffer).unwrap();
        }
    }));

    // UDP Sender
    let handle_sender = std::thread::spawn(move ||{
        let socket = UdpSocket::bind("127.0.0.1:7879").unwrap();
        
        loop{
            let message: Result<Packet, std::sync::mpsc::RecvError> = send_out.recv();

            match message {
                Ok(packet) => {
                    socket.send_to(&packet.payload, "127.0.0.1:7879").unwrap();
                    println!("THREAD: Sended: {:#?}", std::str::from_utf8(&packet.payload[..packet.true_size]).unwrap());
                },
                Err(_) => {
                    println!("Shutting down UDP sending server.");
                }
            };

        }
    });

    let run: bool = true;
    while run {
        thread::sleep(time::Duration::from_millis(1000));   
        println!("Sleep...");

        // Process packets
        let mut next = true;
        while next {
            match receive_out.recv_timeout(Duration::ZERO) {
                Ok(packet) => {
                    let rec_str = std::str::from_utf8(&packet.payload[..packet.true_size]).unwrap();
                    println!("Received: {:#?}", rec_str);

                    if rec_str.eq("shutdown") {
                        println!("Shutting down receiver from main thread.");
                        let (run_mutex, cvar) = &*pair;

                        let mut value = run_mutex.lock().unwrap();
                        *value = false;
                        cvar.notify_one();
                    }

                    let send_string = format!("Hello {:#?}", rec_str);
                    send_in.send(Packet::from_str(&send_string)).unwrap();
                    println!("Sent: {:#?}", send_string);
                },
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    next = false;
                }
                Err(_) => panic!()
            };
        }
    }

    handle_receiver.join().unwrap();
    handle_sender.join().unwrap();
}
