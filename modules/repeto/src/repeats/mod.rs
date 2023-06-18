pub mod inv;

use num::traits::{PrimInt, NumAssign};

pub trait Coordinate: PrimInt + NumAssign {}

impl<T: PrimInt + NumAssign> Coordinate for T {}

