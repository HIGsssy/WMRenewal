/// Player information.
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub house_perc: i32,
    pub suspicion: i32,
    pub disposition: i32,
    pub influence: i32,
    pub customer_fear: i32,
    pub bribe_rate: i32,
    pub businesses_extort: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Nameless".to_string(),
            house_perc: 60,
            suspicion: 0,
            disposition: 0,
            influence: 0,
            customer_fear: 0,
            bribe_rate: 0,
            businesses_extort: 0,
        }
    }
}

impl Player {
    /// Scale stat adjustments so changes have less effect near extremes.
    /// C++ Scale200 algorithm: shift to 0-200, ratio = (200 - adjusted) / 200.
    fn scale200(value: i32, stat: i32) -> i32 {
        let adjusted = (stat + 100).clamp(0, 200) as f64;
        let ratio = (200.0 - adjusted) / 200.0;
        let scaled = value as f64 * ratio;
        if scaled > 0.0 {
            scaled.ceil().max(1.0) as i32
        } else if scaled < 0.0 {
            scaled.floor().min(-1.0) as i32
        } else if value > 0 {
            1
        } else if value < 0 {
            -1
        } else {
            0
        }
    }

    fn limit100(val: i32) -> i32 {
        val.clamp(-100, 100)
    }

    /// Adjust disposition by a scaled amount.
    pub fn adjust_disposition(&mut self, amount: i32) {
        let delta = Self::scale200(amount, self.disposition);
        self.disposition = Self::limit100(self.disposition + delta);
    }

    /// Adjust suspicion by a scaled amount.
    pub fn adjust_suspicion(&mut self, amount: i32) {
        let delta = Self::scale200(amount, self.suspicion);
        self.suspicion = Self::limit100(self.suspicion + delta);
    }

    /// Adjust customer fear by a scaled amount.
    pub fn adjust_customer_fear(&mut self, amount: i32) {
        let delta = Self::scale200(amount, self.customer_fear);
        self.customer_fear = Self::limit100(self.customer_fear + delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale200_at_zero() {
        // At stat 0, adjusted=100, ratio=0.5, so value 10 → 5
        assert_eq!(Player::scale200(10, 0), 5);
    }

    #[test]
    fn test_scale200_minimum() {
        // Even tiny values produce at least ±1
        assert_eq!(Player::scale200(1, 90), 1);
        assert_eq!(Player::scale200(-1, 90), -1);
    }

    #[test]
    fn test_disposition_clamped() {
        let mut p = Player::default();
        for _ in 0..500 {
            p.adjust_disposition(10);
        }
        assert!(p.disposition <= 100);
        for _ in 0..500 {
            p.adjust_disposition(-10);
        }
        assert!(p.disposition >= -100);
    }
}
