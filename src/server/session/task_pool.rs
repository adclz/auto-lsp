use std::panic::RefUnwindSafe;

pub(crate) struct TaskPool<T> {
    pub sender: crossbeam_channel::Sender<T>,
    pub pool: rayon::ThreadPool,
}

impl<T> TaskPool<T> {
    pub(crate) fn new_with_threads(sender: crossbeam_channel::Sender<T>, threads: usize) -> Self {
        Self {
            sender,
            pool: rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap(),
        }
    }

    pub(crate) fn spawn<F>(&self, task: F)
    where
        F: FnOnce(crossbeam_channel::Sender<T>) + Send + 'static,
        T: Send + RefUnwindSafe + 'static,
    {
        let sender = self.sender.clone();
        self.pool.spawn(move || {
            task(sender);
        });
    }

    pub(crate) fn len(&self) -> usize {
        self.pool.current_num_threads()
    }
}
