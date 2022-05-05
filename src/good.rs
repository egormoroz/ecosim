pub const NUM_OF_GOODS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Good {
    Food = 0,
    Clothes = 1,
    Labour = 2,
}

impl Good {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Good::Food => "Food",
            Good::Clothes => "Clothes",
            Good::Labour => "Labour",
        }
    }

    pub fn from_int(x: i32) -> Option<Self> {
        match x {
            0 => Some(Good::Food),
            1 => Some(Good::Clothes),
            2 => Some(Good::Labour),
            _ => None,
        }
    }
}

