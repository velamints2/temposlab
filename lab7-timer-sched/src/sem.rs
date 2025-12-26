use alloc::sync::Arc;
use ostd::sync::{LocalIrqDisabled, SpinLock, WaitQueue};

/// 信号量同步原语
pub struct Semaphore {
    /// 计数值，使用自旋锁保护，并禁用中断以防止死锁
    count: SpinLock<usize, LocalIrqDisabled>,
    /// 等待队列，用于阻塞和唤醒线程
    queue: WaitQueue,
}

impl Semaphore {
    /// 创建一个新的信号量，设置初始计数值
    pub fn new(count: usize) -> Self {
        Self {
            count: SpinLock::new(count),
            queue: WaitQueue::new(),
        }
    }

    /// 阻塞式获取资源 (P 操作)
    pub fn acquire(&self) -> SemaphoreGuard {
        // wait_until 会在闭包返回 true 之前阻塞当前线程
        self.queue.wait_until(|| {
            let mut count = self.count.lock();
            if *count > 0 {
                *count -= 1;
                true
            } else {
                false
            }
        });

        SemaphoreGuard {
            sem: self,
            released: false,
        }
    }

    /// 非阻塞式获取资源
    pub fn try_acquire(&self) -> Option<SemaphoreGuard> {
        let mut count = self.count.lock();
        if *count > 0 {
            *count -= 1;
            Some(SemaphoreGuard {
                sem: self,
                released: false,
            })
        } else {
            None
        }
    }

    /// 释放资源 (V 操作)
    pub fn release(&self) {
        {
            let mut count = self.count.lock();
            *count += 1;
        }
        // 唤醒等待队列中的一个线程
        self.queue.wake_one();
    }
}

/// 信号量资源卫兵，实现 RAII 模式
pub struct SemaphoreGuard<'a> {
    sem: &'a Semaphore,
    released: bool,
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        if !self.released {
            self.sem.release();
            self.released = true;
        }
    }
}

#[cfg(ktest)]
mod test {
    use ostd::prelude::ktest;
    use super::Semaphore;

    #[ktest]
    fn test_semaphore_basic() {
        let sem = Semaphore::new(2);
        
        // 获取两个资源
        let g1 = sem.try_acquire();
        assert!(g1.is_some());
        let g2 = sem.try_acquire();
        assert!(g2.is_some());
        
        // 第三个应该失败
        let g3 = sem.try_acquire();
        assert!(g3.is_none());
        
        // 释放一个
        drop(g1);
        
        // 现在可以获取了
        let g4 = sem.try_acquire();
        assert!(g4.is_some());
    }
}

