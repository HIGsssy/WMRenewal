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
        let mut dispatcher = Self {
            jobs: HashMap::new(),
        };
        dispatcher.register_all();
        dispatcher
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

    /// Register all job implementations.
    fn register_all(&mut self) {
        // General
        self.register(Box::new(general::JobResting));
        self.register(Box::new(general::JobTraining));
        self.register(Box::new(general::JobCleaning));
        self.register(Box::new(general::JobSecurity));
        self.register(Box::new(general::JobAdvertising));
        self.register(Box::new(general::JobMatron));
        self.register(Box::new(general::JobTorturer));
        self.register(Box::new(general::JobExploreCatacombs));
        self.register(Box::new(general::JobBeastCapture));
        self.register(Box::new(general::JobBeastCarer));
        // Brothel
        self.register(Box::new(brothel_jobs::JobWhoreBrothel));
        self.register(Box::new(brothel_jobs::JobWhoreStreets));
        self.register(Box::new(brothel_jobs::JobBrothelStripper));
        self.register(Box::new(brothel_jobs::JobMasseuse));
        // Gambling
        self.register(Box::new(gambling::JobCustomerService));
        self.register(Box::new(gambling::JobWhoreGambHall));
        self.register(Box::new(gambling::JobDealer));
        self.register(Box::new(gambling::JobEntertainment));
        self.register(Box::new(gambling::JobXXXEntertainment));
        // Bar
        self.register(Box::new(bar::JobBarmaid));
        self.register(Box::new(bar::JobWaitress));
        self.register(Box::new(bar::JobStripper));
        self.register(Box::new(bar::JobWhoreBar));
        self.register(Box::new(bar::JobSinger));
        // Movie
        self.register(Box::new(movie::JobFilmBeast));
        self.register(Box::new(movie::JobFilmSex));
        self.register(Box::new(movie::JobFilmAnal));
        self.register(Box::new(movie::JobFilmLesbian));
        self.register(Box::new(movie::JobFilmBondage));
        self.register(Box::new(movie::JobFluffer));
        self.register(Box::new(movie::JobCameraMage));
        self.register(Box::new(movie::JobCrystalPurifier));
        // Community
        self.register(Box::new(community::JobCollectDonations));
        self.register(Box::new(community::JobFeedPoor));
        self.register(Box::new(community::JobMakeItems));
        self.register(Box::new(community::JobSellItems));
        self.register(Box::new(community::JobCommunityService));
        // Drug Lab
        self.register(Box::new(drug_lab::JobVirasPlantFucker));
        self.register(Box::new(drug_lab::JobShroudGrower));
        self.register(Box::new(drug_lab::JobFairyDuster));
        self.register(Box::new(drug_lab::JobDrugDealer));
        // Alchemy
        self.register(Box::new(alchemy::JobFindRegents));
        self.register(Box::new(alchemy::JobBrewPotions));
        self.register(Box::new(alchemy::JobPotionTester));
        // Arena
        self.register(Box::new(arena::JobFightBeasts));
        self.register(Box::new(arena::JobWrestle));
        self.register(Box::new(arena::JobFightToDeath));
        self.register(Box::new(arena::JobFightVolunteers));
        self.register(Box::new(arena::JobCollectBets));
        // Training
        self.register(Box::new(training::JobTeachBDSM));
        self.register(Box::new(training::JobTeachSex));
        self.register(Box::new(training::JobTeachBeast));
        self.register(Box::new(training::JobTeachMagic));
        self.register(Box::new(training::JobTeachCombat));
        self.register(Box::new(training::JobDaycare));
        self.register(Box::new(training::JobSchooling));
        self.register(Box::new(training::JobTeachDancing));
        self.register(Box::new(training::JobTeachService));
        self.register(Box::new(training::JobTrain));
        // Clinic
        self.register(Box::new(clinic::JobDoctor));
        self.register(Box::new(clinic::JobGetAbort));
        self.register(Box::new(clinic::JobPhysicalSurgery));
        self.register(Box::new(clinic::JobHealing));
        self.register(Box::new(clinic::JobRepairShop));
    }
}

// Job trait requires Debug for the dispatcher
impl std::fmt::Debug for dyn Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job({:?})", self.job_type())
    }
}
