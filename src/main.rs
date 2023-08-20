#[derive(Debug)]
struct RingBuffer<T> {
    storage: Vec<T>,
    read_idx: usize,
    write_idx: usize
}

#[derive(Debug)]
struct Full;

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        RingBuffer { 
            storage: vec![T; capacity], 
            read_idx: 1, 
            write_idx: 0 
        }
    }

    fn push(&mut self, item: T) -> Result<(), Full> {
        self.storage.push(item);

        Err(Full)
    }
}

fn main() {
    // let b = RingBuffer::<i32>::new();
    println!("{b:?}");
}
