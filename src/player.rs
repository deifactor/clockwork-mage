use crate::action::*;
use crate::time::*;

#[derive(Clone, Debug)]
pub struct Cast {
    /// When we'll have finished casting.
    pub finish: Timestamp,
    /// Whatever we're currently casting.
    pub action: Action,
}

/// The `Player` stores MP, TP, buffs, and statuses (buffs/debuffs). It does
/// *not* store the rotation to use; that's a separate object that takes in a
/// `&Player`.
///
/// In general, `Player` expects to be called in a loop of `perform()` ->
/// `begin(action)` (if there's an action we should be performing) -> ticking
/// the clock -> repeat.
#[derive(Clone, Debug)]
pub struct Player<'a> {
    mp: i32,
    /// Time until the player can execute their next GCD action (i.e.,
    /// weaponskill/spell). Whenever the player executes an action with a recast
    /// time, this is set to Some(recast). OGCD actions can still be executed as
    /// long as `animation_lock.is_none()`.
    recast_lock: Option<Timestamp>,
    /// Time until the player has finished their current animation. If this is
    /// not None, the player cannot do *anything*.
    animation_lock: Option<Timestamp>,
    /// The current action that the player is casting.
    casting: Option<Cast>,

    clock: &'a Clock,
}

impl<'a> Player<'a> {
    pub fn new(clock: &Clock) -> Player {
        Player {
            mp: 10000,
            recast_lock: None,
            animation_lock: None,
            casting: None,
            clock,
        }
    }

    /// Whether cast/recast/animation locks prevent using the given action.
    pub fn locked(&self, action: Action) -> bool {
        let now = self.clock.now();
        let unlocked =
            (Some(now) >= self.recast_lock || action.is_ogcd()) && Some(now) >= self.animation_lock;
        !unlocked
    }

    pub fn begin(&mut self, action: Action) {
        assert!(
            !self.locked(action),
            "tried to use {:?} when player was in bad state {:?}",
            action,
            self
        );
        assert!(
            action.mp_cost() <= self.mp,
            "MP cost {} for {:?} is greater than MP {}",
            action.mp_cost(),
            action,
            self.mp
        );

        let now = self.clock.now();
        self.recast_lock = Some(now + action.recast_time());
        self.animation_lock = Some(now + action.animation_time());
        self.casting = Some(Cast {
            finish: now + action.cast_time(),
            action,
        });
    }

    /// Performs any action that we're in the middle of casting if its cast timer has reached zero.
    pub fn perform(&mut self) {
        if let Some(casting) = &mut self.casting {
            if self.clock.now() >= casting.finish {
                let action = casting.action;
                self.casting = None;
                self.perform_action(action);
            }
        }
    }

    fn perform_action(&mut self, action: Action) {
        assert!(
            action.mp_cost() <= self.mp,
            "MP cost {} for {:?} is greater than MP {}",
            action.mp_cost(),
            action,
            self.mp
        );
        self.mp -= action.mp_cost();
    }

    pub fn mp(&self) -> i32 {
        self.mp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_unlocked() {
        let clock = Clock::new();
        assert!(!Player::new(&clock).locked(Action::Hit));
    }

    #[test]
    fn starts_locked_after_gcd() {
        let clock = Clock::new();
        let mut player = Player::new(&clock);
        player.begin(Action::Hit);
        assert!(player.locked(Action::Hit));
    }

    #[test]
    fn unlock_timer() {
        let clock = Clock::new();
        let mut player = Player::new(&clock);
        player.begin(Action::Hit);
        for _ in 0..Action::Hit.recast_time().0 - 1 {
            clock.tick();
            player.perform();
        }
        assert!(player.locked(Action::Hit));
        clock.tick();
        player.perform();
        assert!(!player.locked(Action::Hit));
    }

    #[test]
    fn mp_deducted_on_finish() {
        let clock = Clock::new();
        let mut player = Player::new(&clock);
        player.begin(Action::Hit);
        assert_eq!(player.mp(), 10000);
        for _ in 0..Action::Hit.cast_time().0 - 1 {
            clock.tick();
            player.perform();
            assert_eq!(player.mp(), 10000);
        }
        clock.tick();
        player.perform();
        assert_eq!(player.mp(), 9000);
    }
}
