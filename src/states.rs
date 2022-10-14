#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Views {
    TbCombat,
    Driving,
}
pub enum TbCombatPhases {
    PlayerSelectAction,
    AiSelectAction,
}
