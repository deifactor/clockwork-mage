/// The `Player` stores MP, TP, buffs, and statuses (buffs/debuffs). It does
/// *not* store the rotation to use; that's a separate object that takes in a
/// `&Player`.
struct Player {
    mp: i32,

}

impl Player {
    pub fn new() -> Player {
        Player { mp: 15840 }
    }
    pub fn mp(&self) -> i32 { self.mp }
}
