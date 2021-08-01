#[derive(Debug, Copy, Clone, Default)]
pub struct Candidates {
    value: u16,
}

impl Candidates {
    pub fn new(all: bool) -> Candidates {
        Candidates { value: if all { 511 } else { 0 } }
    }

    pub fn value(self) -> u16 {
        self.value
    }

    pub fn get(self, candidate: usize) -> bool {
        ((1 << candidate) & self.value) > 0
    }

    pub fn set_all(&mut self) {
        self.value = 511
    }

    pub fn set(&mut self, candidate: usize) {
        self.value |= 1 << candidate;
    }

    pub fn unset_all(&mut self) {
        self.value = 0
    }

    pub fn unset(&mut self, candidate: usize) {
        self.value &= !(1 << candidate);
    }

    pub fn some(self) -> bool {
        self.value > 0
    }

    pub fn none(self) -> bool {
        self.value == 0
    }

    pub fn count(self) -> usize {
        let mut count = 0;
        let mut flipper = self.value;
        while flipper > 0 {
            flipper &= flipper - 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value() {
        let c = Candidates::new(true);
        assert_eq!(c.value, 511);
        assert_eq!(c.get(5), true);
    }

    #[test]
    fn set_value() {
        let mut c = Candidates::new(true);
        c.unset_all();
        c.set(4);
        assert_eq!(c.get(0), false);
        assert_eq!(c.get(4), true);
    }

    #[test]
    fn mega_test() {
        let mut c = Candidates::new(true);
        assert!(c.some());
        c.unset_all();
        assert!(c.none());
        assert_eq!(c.count(), 0);
        c.set_all();
        assert!(c.some());
        assert_eq!(c.count(), 9);
        for i in 0..9 {
            assert!(c.get(i));
            c.unset(i);
            c.unset(i);
            assert!(!c.get(i));
            c.set(i);
            c.set(i);
            assert!(c.get(i));
        }
        assert!(c.some());
    }
}
