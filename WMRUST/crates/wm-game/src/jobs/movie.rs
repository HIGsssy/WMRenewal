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

stub_job!(JobFilmBeast, JobType::FilmBeast);
stub_job!(JobFilmSex, JobType::FilmSex);
stub_job!(JobFilmAnal, JobType::FilmAnal);
stub_job!(JobFilmLesbian, JobType::FilmLesbian);
stub_job!(JobFilmBondage, JobType::FilmBondage);
stub_job!(JobFluffer, JobType::Fluffer);
stub_job!(JobCameraMage, JobType::CameraMage);
stub_job!(JobCrystalPurifier, JobType::CrystalPurifier);
