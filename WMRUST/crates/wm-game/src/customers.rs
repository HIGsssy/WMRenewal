/// Customer generation for brothels.
#[derive(Debug)]
pub struct CustomerGenerator {
    pub num_customers_per_brothel: Vec<i32>,
}

impl Default for CustomerGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomerGenerator {
    pub fn new() -> Self {
        Self {
            num_customers_per_brothel: Vec::new(),
        }
    }

    pub fn generate_customers(&mut self, _num_brothels: usize) {
        todo!()
    }

    pub fn get_customers(&self, _brothel_id: usize) -> i32 {
        todo!()
    }
}
