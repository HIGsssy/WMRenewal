/// Gold/finance tracking, mirroring C++ cGold/cGoldBase.
#[derive(Debug, Clone, Default)]
pub struct Gold {
    pub cash_on_hand: f64,
    pub bank_balance: f64,
    pub income: IncomeDetail,
    pub expenses: ExpenseDetail,
}

/// Error for gold operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum GoldError {
    #[error("Insufficient funds: need {needed}, have {available}")]
    InsufficientFunds { needed: f64, available: f64 },
}

impl Gold {
    pub fn new(initial_cash: i32) -> Self {
        Self {
            cash_on_hand: initial_cash as f64,
            ..Default::default()
        }
    }

    pub fn total_income(&self) -> f64 {
        self.income.total()
    }

    pub fn total_expenses(&self) -> f64 {
        self.expenses.total()
    }

    pub fn net_income(&self) -> f64 {
        self.total_income() - self.total_expenses()
    }

    pub fn total_profit(&self) -> f64 {
        self.net_income()
    }

    /// Deposit cash into the bank.
    pub fn deposit(&mut self, amount: f64) -> Result<(), GoldError> {
        if amount > self.cash_on_hand {
            return Err(GoldError::InsufficientFunds {
                needed: amount,
                available: self.cash_on_hand,
            });
        }
        self.cash_on_hand -= amount;
        self.bank_balance += amount;
        Ok(())
    }

    /// Withdraw cash from the bank.
    pub fn withdraw(&mut self, amount: f64) -> Result<(), GoldError> {
        if amount > self.bank_balance {
            return Err(GoldError::InsufficientFunds {
                needed: amount,
                available: self.bank_balance,
            });
        }
        self.bank_balance -= amount;
        self.cash_on_hand += amount;
        Ok(())
    }

    /// Check if the player can afford a cash purchase.
    pub fn can_afford(&self, amount: f64) -> bool {
        self.cash_on_hand >= amount
    }

    // -- Income recording methods --

    pub fn add_brothel_work(&mut self, amount: f64) {
        self.income.brothel_work += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_street_work(&mut self, amount: f64) {
        self.income.street_work += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_bar_income(&mut self, amount: f64) {
        self.income.bar_income += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_gambling_profits(&mut self, amount: f64) {
        self.income.gambling_profits += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_item_sales(&mut self, amount: f64) {
        self.income.item_sales += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_slave_sales(&mut self, amount: f64) {
        self.income.slave_sales += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_creature_sales(&mut self, amount: f64) {
        self.income.creature_sales += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_extortion(&mut self, amount: f64) {
        self.income.extortion += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_objective_reward(&mut self, amount: f64) {
        self.income.objective_reward += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_plunder(&mut self, amount: f64) {
        self.income.plunder += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_petty_theft(&mut self, amount: f64) {
        self.income.petty_theft += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_grand_theft(&mut self, amount: f64) {
        self.income.grand_theft += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_catacomb_loot(&mut self, amount: f64) {
        self.income.catacomb_loot += amount;
        self.cash_on_hand += amount;
    }

    /// Bank interest doesn't add to cash — it's in the bank.
    pub fn add_bank_interest(&mut self, amount: f64) {
        self.income.bank_interest += amount;
        self.bank_balance += amount;
    }

    pub fn add_movie_income(&mut self, amount: f64) {
        self.income.movie_income += amount;
        self.cash_on_hand += amount;
    }

    pub fn add_stripper_income(&mut self, amount: f64) {
        self.income.stripper_income += amount;
        self.cash_on_hand += amount;
    }

    /// Misc credit — for gold already in the system (bank transactions, taking from girls).
    pub fn misc_credit(&mut self, amount: f64) {
        self.income.misc_credit += amount;
        self.cash_on_hand += amount;
    }

    // -- Expense recording methods (instant — fail if can't afford) --

    pub fn pay_brothel_cost(&mut self, amount: f64) -> bool {
        if self.cash_on_hand < amount {
            return false;
        }
        self.expenses.brothel_cost += amount;
        self.cash_on_hand -= amount;
        true
    }

    pub fn pay_slave_cost(&mut self, amount: f64) -> bool {
        if self.cash_on_hand < amount {
            return false;
        }
        self.expenses.slave_cost += amount;
        self.cash_on_hand -= amount;
        true
    }

    pub fn pay_item_cost(&mut self, amount: f64) -> bool {
        if self.cash_on_hand < amount {
            return false;
        }
        self.expenses.item_cost += amount;
        self.cash_on_hand -= amount;
        true
    }

    pub fn pay_consumable_cost(&mut self, amount: f64) -> bool {
        if self.cash_on_hand < amount {
            return false;
        }
        self.expenses.consumable_cost += amount;
        self.cash_on_hand -= amount;
        true
    }

    // -- Expense recording methods (forced — paid whether or not you can afford it) --

    pub fn charge_goon_wages(&mut self, amount: f64) {
        self.expenses.goon_wages += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_staff_wages(&mut self, amount: f64) {
        self.expenses.staff_wages += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_girl_support(&mut self, amount: f64) {
        self.expenses.girl_support += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_girl_training(&mut self, amount: f64) {
        self.expenses.girl_training += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_building_upkeep(&mut self, amount: f64) {
        self.expenses.building_upkeep += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_bar_upkeep(&mut self, amount: f64) {
        self.expenses.bar_upkeep += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_casino_upkeep(&mut self, amount: f64) {
        self.expenses.casino_upkeep += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_advertising(&mut self, amount: f64) {
        self.expenses.advertising_costs += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_bribes(&mut self, amount: f64) {
        self.expenses.bribes += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_fines(&mut self, amount: f64) {
        self.expenses.fines += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_tax(&mut self, amount: f64) {
        self.expenses.tax += amount;
        self.cash_on_hand -= amount;
    }

    pub fn charge_rival_raids(&mut self, amount: f64) {
        self.expenses.rival_raids += amount;
        self.cash_on_hand -= amount;
    }

    /// Misc debit — requires cash on hand.
    pub fn misc_debit(&mut self, amount: f64) -> bool {
        if self.cash_on_hand < amount {
            return false;
        }
        self.cash_on_hand -= amount;
        true
    }

    /// Reset weekly counters (end of turn).
    pub fn reset_weekly(&mut self) {
        self.income = IncomeDetail::default();
        self.expenses = ExpenseDetail::default();
    }

    /// Process end-of-week: settle delayed transactions, then reset.
    pub fn week_end(&mut self) {
        // Income/expenses have already been applied to cash_on_hand
        // via the individual methods. Just reset the tracking.
        self.reset_weekly();
    }
}

/// Breakdown of all income sources.
#[derive(Debug, Clone, Default)]
pub struct IncomeDetail {
    pub brothel_work: f64,
    pub street_work: f64,
    pub bar_income: f64,
    pub gambling_profits: f64,
    pub item_sales: f64,
    pub slave_sales: f64,
    pub creature_sales: f64,
    pub extortion: f64,
    pub objective_reward: f64,
    pub plunder: f64,
    pub petty_theft: f64,
    pub grand_theft: f64,
    pub catacomb_loot: f64,
    pub bank_interest: f64,
    pub movie_income: f64,
    pub stripper_income: f64,
    pub misc_credit: f64,
}

impl IncomeDetail {
    pub fn total(&self) -> f64 {
        self.brothel_work
            + self.street_work
            + self.bar_income
            + self.gambling_profits
            + self.item_sales
            + self.slave_sales
            + self.creature_sales
            + self.extortion
            + self.objective_reward
            + self.plunder
            + self.petty_theft
            + self.grand_theft
            + self.catacomb_loot
            + self.bank_interest
            + self.movie_income
            + self.stripper_income
            + self.misc_credit
    }
}

/// Breakdown of all expense categories.
#[derive(Debug, Clone, Default)]
pub struct ExpenseDetail {
    pub brothel_cost: f64,
    pub slave_cost: f64,
    pub item_cost: f64,
    pub consumable_cost: f64,
    pub goon_wages: f64,
    pub staff_wages: f64,
    pub girl_support: f64,
    pub girl_training: f64,
    pub building_upkeep: f64,
    pub bar_upkeep: f64,
    pub casino_upkeep: f64,
    pub advertising_costs: f64,
    pub bribes: f64,
    pub fines: f64,
    pub tax: f64,
    pub rival_raids: f64,
}

impl ExpenseDetail {
    pub fn total(&self) -> f64 {
        self.brothel_cost
            + self.slave_cost
            + self.item_cost
            + self.consumable_cost
            + self.goon_wages
            + self.staff_wages
            + self.girl_support
            + self.girl_training
            + self.building_upkeep
            + self.bar_upkeep
            + self.casino_upkeep
            + self.advertising_costs
            + self.bribes
            + self.fines
            + self.tax
            + self.rival_raids
    }
}
