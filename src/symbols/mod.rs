pub mod box_drawing;

use crate::symbol::Symbol;

/*
///
pub fn pattern(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false],
        ],
    }
}
 */

///
pub fn block(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
        ],
    }
}

///
pub fn dot(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, false, false, false, true, false, false, false, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
        ],
    }
}

///
pub fn grid(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            // TODO
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            //
            &[true, true, true, true, true, true, true, true, true],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[true, true, true, true, true, true, true, true, true],
            //
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
            &[false, false, true, false, false, false, true, false, false],
        ],
    }
}

///
pub fn hash(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            //
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            //
            &[false, true, true, true, true, true, true, true, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, true, true, true, true, true, true, true, false],
            //
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            //
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
        ],
    }
}

///
pub fn heart(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            //
            &[false, true, true, true, false, true, true, true, false],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[true, true, true, true, true, true, true, true, true],
            &[false, true, true, true, true, true, true, true, false],
            &[false, false, true, true, true, true, true, false, false],
            &[false, false, false, true, true, true, false, false, false],
            &[false, false, false, false, true, false, false, false, false],
            //
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
        ],
    }
}

///
pub fn pattern(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
            &[false, true, false, true, false, true, false, true, false],
            &[true, false, true, false, true, false, true, false, true],
        ],
    }
}

///
pub fn pattern2(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[false, true, false, false, true, false, false, true, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, true, false, false, true, false, false, true, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, true, false, false, true, false, false, true, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, true, false, false, true, false, false, true, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, true, false, false, true, false, false, true, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
        ],
    }
}

///
pub fn wave(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            // TODO
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[false, false, true, true, false, false, false, false, false],
            &[false, true, false, false, true, false, false, true, false],
            &[false, false, false, false, false, true, true, false, false],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
        ],
    }
}
