/// Represents an individual action that the player uses (spell, weaponskill,
/// oGCD, etc.).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Hit,
    Recharge
}
