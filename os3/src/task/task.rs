//! Types related to task management

use super::TaskContext;
use crate::{config::MAX_SYSCALL_NUM, timer::get_time_us};

#[derive(Copy, Clone, Debug, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    status: TaskStatus,
    pub ctx: TaskContext,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    start_time: usize,
    // LAB1: Add whatever you need about the Task.
}

impl TaskControlBlock {
    #[inline]
    pub const fn new() -> Self {
        Self {
            status: TaskStatus::UnInit,
            ctx: TaskContext::zero_init(),
            syscall_times: [0; MAX_SYSCALL_NUM],
            start_time: 0,
        }
    }
    #[inline]
    pub const fn get_status(&self) -> TaskStatus {
        self.status
    }

    #[inline]
    pub const fn get_start_time(&self) -> usize {
        self.start_time
    }

    pub fn set_run(&mut self) {
        match self.status {
            TaskStatus::Ready => {
                if self.start_time == 0 {
                    self.start_time = get_time_us()
                }
            }
            _ => unreachable!(),
        }
        self.status = TaskStatus::Running;
    }

    pub fn set_ready(&mut self) {
        self.status = match self.status {
            TaskStatus::Running | TaskStatus::UnInit => TaskStatus::Ready,
            _ => unreachable!(),
        };
    }

    pub fn set_exit(&mut self) {
        self.status = TaskStatus::Exited;
    }

    pub fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.syscall_times
    }
}
