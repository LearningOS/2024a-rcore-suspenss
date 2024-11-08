use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct DeadLock {
    pub avail: Vec<usize>,
    pub alloc: Vec<Vec<usize>>,
    pub need: Vec<Vec<usize>>,
}

impl DeadLock {
    /// new a detector
    pub fn new() -> Self {
        Self {
            avail: Vec::new(),
            alloc: vec![vec![]],
            need: vec![vec![]],
        }
    }

    /// add resource
    pub fn add_resource(&mut self, count: usize) {
        self.avail.push(count);
        self.alloc.iter_mut().for_each(|v| v.push(0));
        self.need.iter_mut().for_each(|v| v.push(0));
    }

    /// add thread
    pub fn add_thread(&mut self) {
        let res_num = self.avail.len();
        self.alloc.push(vec![0; res_num]);
        self.need.push(vec![0; res_num]);
    }

    /// release resource
    pub fn release_res(&mut self, i: usize, j: usize) {
        self.avail[j] += 1;
        self.alloc[i][j] -= 1;
    }

    pub fn alloc_res(&mut self, i: usize, j: usize) {
        self.need[i][j] -= 1;
        self.alloc[i][j] += 1;
        self.avail[j] -= 1;
    }

    /// check
    pub fn detect(&self) -> bool {
        let thread_num = self.alloc.len();
        let mut work = self.avail.clone();
        let mut finish = vec![false; thread_num];

        loop {
            let satsify = (0..thread_num).find(|&i| {
                (finish[i] == false) && self.need[i].iter().zip(work.iter()).all(|(&x, &y)| x <= y)
            });

            if let Some(i) = satsify {
                for j in 0..work.len() {
                    work[j] += self.alloc[i][j];
                }
                finish[i] = true;
            } else {
                break;
            }
        }

        info!("{}", finish.iter().all(|x| *x));
        info!("{:?}", self);
        !finish.iter().all(|x| *x)
    }
}

/// deadlock checker
pub struct DeadLockChecker {
    /// mutex
    pub mutex: DeadLock,
    /// semaphore
    pub semaphore: DeadLock,
}

impl DeadLockChecker {
    /// new
    pub fn new() -> Self {
        Self {
            mutex: DeadLock::new(),
            semaphore: DeadLock::new(),
        }
    }
}
