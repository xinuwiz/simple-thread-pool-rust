use std::thread::JoinHandle;

pub struct ThreadPool {
    vec: Vec<JoinHandle<()>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool {
            vec: Vec::with_capacity(size)
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where F: FnOnce() + Send + 'static {
        let x = self.vec.len();
        println!("{x}");
        let thread = std::thread::spawn(f);
        self.vec.push(thread);
    }

    pub fn clean(&mut self) {
        self.vec.retain(move |handler| {
            !handler.is_finished()
        })
    }
}

fn main() {

}