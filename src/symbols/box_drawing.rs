use crate::symbol::Symbol;

///
pub fn double_bl(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, true, true, true],
            &[false, false, false, true, false, false, false, false, false],
            &[false, false, false, true, true, true, true, true, true],
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
pub fn double_br(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[true, true, true, true, false, true, false, false, false],
            &[false, false, false, false, false, true, false, false, false],
            &[true, true, true, true, true, true, false, false, false],
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
pub fn double_horizontal(identifier: &'static str) -> Symbol {
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
            &[true, true, true, true, true, true, true, true, true],
            &[
                false, false, false, false, false, false, false, false, false,
            ],
            &[true, true, true, true, true, true, true, true, true],
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
pub fn double_tl(identifier: &'static str) -> Symbol {
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
            &[false, false, false, true, true, true, true, true, true],
            &[false, false, false, true, false, false, false, false, false],
            &[false, false, false, true, false, true, true, true, true],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
        ],
    }
}

///
pub fn double_tr(identifier: &'static str) -> Symbol {
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
            &[true, true, true, true, true, true, false, false, false],
            &[false, false, false, false, false, true, false, false, false],
            &[true, true, true, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
        ],
    }
}

///
pub fn double_vertical(identifier: &'static str) -> Symbol {
    Symbol {
        identifier,
        rows: &[
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
            &[false, false, false, true, false, true, false, false, false],
        ],
    }
}
