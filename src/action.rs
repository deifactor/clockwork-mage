/// Represents an individual action that the player uses (spell, weaponskill,
/// oGCD, etc.).
use crate::time::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Hit,
    Recharge,
}

impl Action {
    pub fn cast_time(self) -> Duration {
        match self {
            Action::Hit => Duration(150),
            Action::Recharge => Duration(250),
        }
    }

    pub fn recast_time(self) -> Duration {
        Duration(250)
    }

    pub fn animation_time(self) -> Duration {
        Duration(50)
    }

    pub fn mp_cost(self) -> i32 {
        match self {
            Action::Hit => 1000,
            Action::Recharge => -2000,
        }
    }

    pub fn is_ogcd(self) -> bool {
        false
    }
}
