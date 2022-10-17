#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Views {
    TbCombat,
    Driving,
}
pub enum TbCombatPhases {
    SetCanAttack,
    SetupGui,
    PlayerSelectAction,
    AiSelectAction,
}
