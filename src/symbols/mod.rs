pub mod box_drawing;

use crate::Symbol;

/*
///
pub fn pattern() -> Symbol {
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
pub fn block() -> Symbol {
    &[
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
    ]
}

///
pub fn dot() -> Symbol {
    &[
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
    ]
}

///
pub fn grid() -> Symbol {
    &[
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
    ]
}

///
pub fn hash() -> Symbol {
    &[
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
    ]
}

///
pub fn heart() -> Symbol {
    &[
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
    ]
}

///
pub fn pattern() -> Symbol {
    &[
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
    ]
}

///
pub fn pattern2() -> Symbol {
    &[
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
    ]
}

///
pub fn wave() -> Symbol {
    &[
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
    ]
}
