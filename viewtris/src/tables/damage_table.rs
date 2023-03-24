use std::{collections::HashMap, ops::Index};

use crate::tetromino::MinoVariant;

pub struct Attack {
    pub combo: Option<u16>,
    pub b2b: Option<u16>,
    pub piece: Option<MinoVariant>,
    pub lines: Option<u8>,
    pub spin: Option<bool>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AttackKey {
    pub combo: u16,
    pub b2b: u16,
    pub piece: MinoVariant,
    pub lines: u8,
    pub spin: bool,
}

impl Attack {
    fn matches(&self, key: &AttackKey) -> bool {
        self.combo.map(|combo| combo == key.combo).unwrap_or(true)
            && self.b2b.map(|b2b| b2b == key.b2b).unwrap_or(true)
            && self.piece.map(|piece| piece == key.piece).unwrap_or(true)
            && self.lines.map(|lines| lines == key.lines).unwrap_or(true)
            && self.spin.map(|spin| spin == key.spin).unwrap_or(true)
    }
}

/// **Caution**: Damage table is defined *with an ordering*
///
/// The damage table describes how much damage should be sent per attack. Since many, *many* kinds
/// of attacks deal the same amount of damage (usually anything that isn't a T and isn't a spin),
/// this lookup table is defined in a tiered way -- completely specific entries can be defined for
/// when all entries in [AttackKey] matter. More general cases will be checked after more specific
/// cases. They will be checked in linear time, however, so it's a good idea to keep the size of
/// that set down.
pub struct DamageTable {
    /// One entry describes the damage output of a group of attacks. Groups are checked in the order
    /// in the vec, so groups should be ordered from most to least specific.
    pub general: Vec<(Attack, u16)>,
    pub specific: HashMap<AttackKey, u16>,
}

impl DamageTable {
    fn get_uncopied(&self, index: &AttackKey) -> Option<&u16> {
        self.specific.get(index).or(self
            .general
            .iter()
            .find_map(|(attack, output)| attack.matches(index).then_some(output)))
    }

    pub fn get(&self, index: &AttackKey) -> Option<u16> {
        self.get_uncopied(index).copied()
    }
}

impl Index<&AttackKey> for DamageTable {
    type Output = u16;

    fn index(&self, index: &AttackKey) -> &Self::Output {
        self.get_uncopied(index).unwrap()
    }
}
