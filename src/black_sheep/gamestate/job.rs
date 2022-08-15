
use std::{thread::{JoinHandle, self}, sync::{Mutex, Arc, atomic::AtomicBool, Condvar}};
use core::sync::atomic::Ordering::SeqCst;

struct Job {
    join_h: JoinHandle<()>,
    cond_pair: Arc<(Mutex<bool>, Condvar)>,
    shut_down: Arc<AtomicBool>,
}

impl Job {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + 'static,
    {
        let shut_down = Arc::new(AtomicBool::new(false));
        let shut_down_c = Arc::clone(&shut_down);

        let cond_pair = Arc::new((Mutex::new(false), Condvar::new()));
        let cond_pair_c = Arc::clone(&cond_pair);

        let join_h = thread::spawn(move || {
            let (s_lock, s_cvar) = &*cond_pair_c;

            'main_loop: loop {
                let mut running = s_lock.lock().unwrap();
                while !*running {
                    running = s_cvar.wait(running).unwrap();
                    if shut_down_c.load(SeqCst) {
                        break 'main_loop;
                    }
                }

                f();

                *running = false;
                s_cvar.notify_one();
            };
        });

        Job {
            join_h,
            cond_pair,
            shut_down,
        }
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        self.shut_down.store(true, SeqCst);
        self.cond_pair.1.notify_one();
    }
}