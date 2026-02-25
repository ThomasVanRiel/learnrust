#[derive(Debug)]
pub enum FilterOp {
    Eq,
    Ne,
    Gt,
    St,
    Ge,
    Se,
}

impl FilterOp {
    pub fn compare<T: std::cmp::PartialOrd>(&self, rhs: T, lhs: T) -> bool {
        match self {
            FilterOp::Eq => rhs == lhs,
            FilterOp::Ne => rhs != lhs,
            FilterOp::Gt => rhs > lhs,
            FilterOp::St => rhs < lhs,
            FilterOp::Ge => rhs >= lhs,
            FilterOp::Se => rhs <= lhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_matches_equal_values() {
        assert!(FilterOp::Eq.compare(5, 5));
    }

    #[test]
    fn eq_rejects_unequal_values() {
        assert!(!FilterOp::Eq.compare(5, 6));
    }

    #[test]
    fn eq_matches_greater_values() {
        assert!(FilterOp::Gt.compare(10, 5));
    }
}
