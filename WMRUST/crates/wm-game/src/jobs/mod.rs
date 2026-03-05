pub mod general;
pub mod brothel_jobs;
pub mod gambling;
pub mod bar;
pub mod movie;
pub mod community;
pub mod drug_lab;
pub mod alchemy;
pub mod arena;
pub mod training;
pub mod clinic;

use std::collections::HashMap;

use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;

/// Result of processing a single girl's job for one shift.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct JobResult {
    pub gold_earned: i32,
    pub events: Vec<String>,
    pub stat_changes: Vec<(Stat, i32)>,
    pub skill_changes: Vec<(Skill, i32)>,
}


/// Trait for implementing a job processor.
pub trait Job {
    fn job_type(&self) -> JobType;
    fn process(
        &self,
        girl: &mut Girl,
        brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult;
}

/// Dispatches job processing to the correct implementation.
#[derive(Debug)]
pub struct JobDispatcher {
    jobs: HashMap<JobType, Box<dyn Job>>,
}

impl Default for JobDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl JobDispatcher {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn register(&mut self, job: Box<dyn Job>) {
        self.jobs.insert(job.job_type(), job);
    }

    pub fn process(
        &self,
        job_type: JobType,
        girl: &mut Girl,
        brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        if let Some(job) = self.jobs.get(&job_type) {
            job.process(girl, brothel, rng)
        } else {
            JobResult::default()
        }
    }
}

// Job trait requires Debug for the dispatcher
impl std::fmt::Debug for dyn Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job({:?})", self.job_type())
    }
}
