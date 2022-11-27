use core::{time, panic};
use std::{net::UdpSocket, thread, time::Duration, sync::{Arc, Mutex}};

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

pub fn start_udp_server(){
    let (receive_in, receive_out) = std::sync::mpsc::channel();
    let (send_in, send_out) = std::sync::mpsc::channel();

    let kill_recv = Arc::new(Mutex::new(false));
    let kill_recv_copy = kill_recv.clone();

    // UDP Receiver
    let handle_receiver = std::thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:7878").unwrap();
        socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();

        loop {
            let mut buffer = [0; 1024];
            
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) =>  {
                    println!("THREAD: Received: {:#?}", std::str::from_utf8(&buffer[..size]).unwrap());
                    receive_in.send(Packet{payload: buffer, true_size: size}).unwrap();
                },
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::TimedOut => {
                            let kill = kill_recv_copy.lock().unwrap();
                            if *kill {
                                println!("UDP receiver thread shutting down.");
                                break;
                            }
                        },
                        _ => panic!()
                    }
                }
            };
        }
    });

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
                    break;
                }
            };

        }
    });

    let mut run: bool = true;
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

                    if rec_str.eq("shutdown\n\n") {
                        println!("Shutting down receiver from main thread.");

                        let mut value = kill_recv.lock().unwrap();
                        *value = true;
                        run = false;
                    }else{
                        let send_string = format!("Hello {:#?}", rec_str);
                        send_in.send(Packet::from_str(&send_string)).unwrap();
                        println!("Sent: {:#?}", send_string);
                    }
                },
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    next = false;
                }
                Err(_) => panic!()
            };
        }
    }

    drop(send_in);

    handle_receiver.join().unwrap();
    handle_sender.join().unwrap();
}
