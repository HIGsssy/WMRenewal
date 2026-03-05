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

stub_job!(JobTeachBDSM, JobType::TeachBDSM);
stub_job!(JobTeachSex, JobType::TeachSex);
stub_job!(JobTeachBeast, JobType::TeachBeast);
stub_job!(JobTeachMagic, JobType::TeachMagic);
stub_job!(JobTeachCombat, JobType::TeachCombat);
stub_job!(JobDaycare, JobType::Daycare);
stub_job!(JobSchooling, JobType::Schooling);
stub_job!(JobTeachDancing, JobType::TeachDancing);
stub_job!(JobTeachService, JobType::TeachService);
stub_job!(JobTrain, JobType::Train);
