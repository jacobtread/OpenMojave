use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(i8)]
pub enum Skill {
    None = -1,
    Barter = 0,
    // Unused
    BigGuns = 1,
    EnergyWeapons = 2,
    Explosives = 3,
    LOckpick = 4,
    Medicine = 5,
    MeleeWeapons = 6,
    Repair = 7,
    Science = 8,
    Guns = 9,
    Sneak = 10,
    Speech = 11,
    Survival = 12,
    Unarmed = 13,
}
