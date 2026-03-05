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

stub_job!(JobDoctor, JobType::Doctor);
stub_job!(JobGetAbort, JobType::GetAbort);
stub_job!(JobPhysicalSurgery, JobType::PhysicalSurgery);
stub_job!(JobHealing, JobType::Healing);
stub_job!(JobRepairShop, JobType::RepairShop);
