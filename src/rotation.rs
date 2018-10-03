use crate::action::Action;
use crate::player::Player;

/// A `Rotation` dictates the sequence of actions that the player takes.
pub trait Rotation {
    /// Determines what the player should do next.
    fn action(&mut self, player: &Player) -> Option<Action>;
}

/// Never does anything.
pub struct Empty {}
impl Rotation for Empty {
    fn action(&mut self, player: &Player) -> Option<Action> {
        None
    }
}

/// Performs the same sequence of actions over and over.
pub struct Repeat {
    actions: Vec<Action>,
    current: usize,
}
impl Repeat {
    pub fn new(actions: Vec<Action>) -> Repeat {
        Repeat {
            actions,
            current: 0,
        }
    }
}
impl Rotation for Repeat {
    fn action(&mut self, player: &Player) -> Option<Action> {
        let action = self.actions[self.current];
        self.current = (self.current + 1) % self.actions.len();
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::Clock;

    #[test]
    fn empty() {
        let clock = Clock::new();
        let player = Player::new(&clock);
        assert_eq!(Empty {}.action(&player), None);
    }

    #[test]
    fn repeat() {
        let clock = Clock::new();
        let player = Player::new(&clock);
        let mut rotation = Repeat::new(vec![Action::Hit, Action::Hit, Action::Recharge]);
        assert_eq!(rotation.action(&player), Some(Action::Hit));
        assert_eq!(rotation.action(&player), Some(Action::Hit));
        assert_eq!(rotation.action(&player), Some(Action::Recharge));
        assert_eq!(rotation.action(&player), Some(Action::Hit));
    }
}
