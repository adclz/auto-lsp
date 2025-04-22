/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

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
