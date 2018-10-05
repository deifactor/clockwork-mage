use crate::action::Action;
use crate::player::Player;
use crate::rotation::Rotation;
use crate::target::Target;
use crate::time::*;
use std::rc::Rc;

/// Performs an entire simulated rotation on a target. This is the main struct
/// for clockwork-mage; everything on top of this is just I/O-type things, like
/// setting up loggers, reading configuration, and the like.
struct Simulator {
    player: Player,
    target: Target,
    clock: Rc<Clock>,
    rotation: Box<dyn Rotation>,
    event_log: Vec<Event>,
}

impl Simulator {
    pub fn new<R: Rotation + 'static>(rotation: R) -> Simulator {
        let clock = Rc::new(Clock::new());
        let player = Player::new(&clock);
        let target = Target {};
        Simulator {
            player,
            target,
            clock,
            rotation: Box::new(rotation),
            event_log: Vec::new(),
        }
    }

    /// Simulates a single time step. In order, this performs any action that
    /// finished casting, enqueues the next action, then ticks the clock.
    pub fn step(&mut self) {
        if let Some(action) = self.player.perform() {
            self.log_event(EventKind::Perform(action));
        }
        if let Some(action) = self.rotation.action(&self.player) {
            self.player.begin(action);
            self.log_event(EventKind::Begin(action));
        }
        self.clock.tick();
    }

    pub fn event_log(&self) -> &Vec<Event> {
        &self.event_log
    }

    pub fn now(&self) -> Timestamp {
        self.clock.now()
    }

    fn log_event(&mut self, kind: EventKind) {
        self.event_log.push(Event {
            timestamp: self.now(),
            kind,
        })
    }
}

/// An `Event` represents everything that can happen in the game. This does not
/// include any rotation-internal logic (like 'we've cast our third Fire IV,
/// time to Fire I'). Think of it as roughly analogous to the battle log, except
/// with DoT ticks and the like.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EventKind {
    Begin(Action),
    Perform(Action),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Event {
    pub kind: EventKind,
    pub timestamp: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rotation;

    #[test]
    fn starts_at_zero() {
        assert_eq!(Simulator::new(rotation::Empty {}).now(), Timestamp(0))
    }

    #[test]
    fn can_begin_on_same_tick_as_perform() {
        let action = Action::Recharge;
        let rotation = rotation::Repeat::new(vec![action]);
        let mut simulator = Simulator::new(rotation);
        assert_eq!(action.cast_time(), action.recast_time());
        while simulator.now() < Timestamp(0) + action.cast_time() {
            simulator.step();
        }
        assert_eq!(
            &vec![Event {
                kind: EventKind::Begin(Action::Recharge),
                timestamp: Timestamp(0)
            }],
            simulator.event_log()
        );
        simulator.step();
        assert_eq!(
            &vec![
                Event {
                    kind: EventKind::Begin(Action::Recharge),
                    timestamp: Timestamp(0)
                },
                Event {
                    kind: EventKind::Perform(Action::Recharge),
                    timestamp: Timestamp(250)
                },
                Event {
                    kind: EventKind::Begin(Action::Recharge),
                    timestamp: Timestamp(250)
                }
            ],
            simulator.event_log()
        )
    }
}
