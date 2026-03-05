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

stub_job!(JobBarmaid, JobType::Barmaid);
stub_job!(JobWaitress, JobType::Waitress);
stub_job!(JobStripper, JobType::Stripper);
stub_job!(JobWhoreBar, JobType::WhoreBar);
stub_job!(JobSinger, JobType::Singer);
