use wm_core::enums::JobType;
use wm_core::girl::Girl;

use crate::brothel::Brothel;

use super::{Job, JobResult};

macro_rules! stub_job {
    ($name:ident, $variant:expr) => {
        pub struct $name;
        impl Job for $name {
            fn job_type(&self) -> JobType { $variant }
            fn process(&self, _girl: &mut Girl, _brothel: &Brothel, _rng: &mut dyn rand::RngCore) -> JobResult {
                todo!()
            }
        }
    };
}

stub_job!(JobResting, JobType::Resting);
stub_job!(JobTraining, JobType::Training);
stub_job!(JobCleaning, JobType::Cleaning);
stub_job!(JobSecurity, JobType::Security);
stub_job!(JobAdvertising, JobType::Advertising);
stub_job!(JobMatron, JobType::Matron);
stub_job!(JobTorturer, JobType::Torturer);
stub_job!(JobExploreCatacombs, JobType::ExploreCatacombs);
stub_job!(JobBeastCapture, JobType::BeastCapture);
stub_job!(JobBeastCarer, JobType::BeastCarer);
