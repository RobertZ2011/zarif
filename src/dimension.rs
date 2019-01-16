pub enum Dynamic {}
pub enum D1 {}
pub enum D2 {}
pub enum D3 {}
pub enum D4 {}
pub enum D5 {}
pub enum D6 {}
pub enum D7 {}
pub enum D8 {}
pub enum D9 {}
pub enum D10 {}
pub enum D11 {}
pub enum D12 {}
pub enum D13 {}
pub enum D14 {}
pub enum D15 {}

pub trait Dimension {
    fn static_size() -> usize;
}

impl Dimension for Dynamic {
    fn static_size() -> usize {
        0
    }
}

impl Dimension for D1 {
    fn static_size() -> usize {
        1
    }
}

impl Dimension for D2 {
    fn static_size() -> usize {
        2
    }
}

impl Dimension for D3 {
    fn static_size() -> usize {
        3
    }
}

impl Dimension for D4 {
    fn static_size() -> usize {
        4
    }
}

impl Dimension for D5 {
    fn static_size() -> usize {
        5
    }
}

impl Dimension for D6 {
    fn static_size() -> usize {
        6
    }
}

impl Dimension for D7 {
    fn static_size() -> usize {
        7
    }
}

impl Dimension for D8 {
    fn static_size() -> usize {
        8
    }
}

impl Dimension for D9 {
    fn static_size() -> usize {
        9
    }
}

impl Dimension for D10 {
    fn static_size() -> usize {
        10
    }
}

impl Dimension for D11 {
    fn static_size() -> usize {
        11
    }
}

impl Dimension for D12 {
    fn static_size() -> usize {
        12
    }
}

impl Dimension for D13 {
    fn static_size() -> usize {
        13
    }
}

impl Dimension for D14 {
    fn static_size() -> usize {
        14
    }
}

impl Dimension for D15 {
    fn static_size() -> usize {
        15
    }
}