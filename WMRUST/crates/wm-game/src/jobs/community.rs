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

stub_job!(JobCollectDonations, JobType::CollectDonations);
stub_job!(JobFeedPoor, JobType::FeedPoor);
stub_job!(JobMakeItems, JobType::MakeItems);
stub_job!(JobSellItems, JobType::SellItems);
stub_job!(JobCommunityService, JobType::CommunityService);
