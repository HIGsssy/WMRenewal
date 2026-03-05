/// Player information.
#[derive(Debug, Clone, Default)]
pub struct Player {
    pub name: String,
    pub house_perc: i32,
    pub suspicion: i32,
    pub disposition: i32,
    pub influence: i32,
    pub customers_daily: i32,
}
