#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::{HashMap}, thread::Thread, net::UdpSocket};


mod primitives;
use primitives::position3::Position3;

mod server;
use server::tcp_server;

use crate::server::{tcp_server::accept_connections, udp_server::start_udp_server};

struct Player {
    id: u64,
    name: String
}

#[derive(Clone, Copy)]
struct Field {
    position: Position3
}


struct Chunk<const T: usize> {
    fields: [Field; T],
    origin: Position3
}

impl<const T: usize> Chunk<T>{
    fn create_chunk() -> Chunk<T>{
        Chunk::<T>{
            fields: [
                Field{position: Position3::origin()}; 
                T
            ],
            origin: Position3::origin()
        }
    }
}

struct World {
    chunks: HashMap<Position3, Chunk<32>>
}

struct ThreadMap {
    thread_childs: Vec<Vec<usize>>,
    thread_deps: Vec<Vec<(usize, bool)>>,

    thread_id: usize
}

impl ThreadMap {
    fn register_thread(&mut self) -> usize {
        self.thread_id += 1;
        self.thread_childs.push(vec![]);
        self.thread_deps.push(vec![]);

        return self.thread_id;
    }

    fn add_dependency(&mut self, id: usize, before_id: usize){
        self.thread_childs[before_id].push(id);
        self.thread_deps[id].push((before_id, false));
    }

    fn execute_map(&mut self){
        // Find all with no dependency
    }

    fn create() -> ThreadMap {
        ThreadMap { 
            thread_childs: vec![], 
            thread_deps: vec![], 
            thread_id: 0 
        }
    }
}


fn main() {
    // Chunk constants
    // const CHUNK_AXIS_SIZE: u64 = 32;
    // const CHUNK_SIZE: usize = CHUNK_AXIS_SIZE.pow(3) as usize;
    // Base chunk
    //let mut first_chunk = Chunk::<CHUNK_SIZE>{fields: [Field{position: Position::origin()}; CHUNK_SIZE]};

    start_udp_server();
    //accept_connections();

    println!("Hello, world!");
}