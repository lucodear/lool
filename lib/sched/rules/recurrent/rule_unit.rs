use num_traits::PrimInt;

/// 🧉 » a recurrence rule unit
///
/// represents a single rule unit that can be used to match a value
#[derive(Clone, Debug)]
pub enum Rule<T>
where
    T: PrimInt,
{
    /// a single value
    Val(T),
    /// range from `start` to `end` with `step` increment
    Range(T, T, T),
    /// a list of single values
    Many(Vec<T>),
    /// a list of ranges
    Ranges(Vec<(T, T, T)>),
}

impl<T> Rule<T>
where
    T: PrimInt,
{
    /// 🧉 » check if the value matches this `Rule` Unit
    pub fn matches(&self, value: T) -> bool {
        Self::_matches(self, value)
    }

    /// 🚧 internal
    fn _matches(matcher: &Rule<T>, value: T) -> bool {
        match matcher {
            Rule::Val(v) => value == *v,
            Rule::Range(start, end, step) => {
                if *start == *end {
                    return value == *start;
                }

                if *step == T::zero() || *step == T::one() {
                    if *start < *end {
                        return value >= *start && value <= *end;
                    } else {
                        return value >= *start || value <= *end;
                    }
                } else {
                    if *start < *end {
                        return value >= *start
                            && value <= *end
                            && (value - *start) % *step == T::zero();
                    } else {
                        return (value >= *start || value <= *end)
                            && (*start - value) % *step == T::zero();
                    }
                }
            }
            Rule::Many(matcher) => matcher.iter().any(|v| Self::_matches(&Rule::Val(*v), value)),
            Rule::Ranges(matcher) => matcher
                .iter()
                .any(|(start, end, step)| Self::_matches(&Rule::Range(*start, *end, *step), value)),
        }
    }

    pub(crate) fn value_is_between(&self, min: T, max: T) -> bool {
        match self {
            Rule::Val(v) => *v >= min && *v <= max,
            Rule::Range(start, end, _) => *start >= min && *end <= max,
            Rule::Many(values) => values.iter().all(|v| *v >= min && *v <= max),
            Rule::Ranges(ranges) => {
                ranges.iter().all(|(start, end, _)| *start >= min && *end <= max)
            }
        }
    }
}

/// 🧉 » create a `Rule` that will match a single value
pub fn val<T: PrimInt>(value: T) -> Rule<T> {
    Rule::Val(value)
}

/// 🧉 » create a `Rule` that will match a range of values
pub fn range<T: PrimInt>(start: T, end: T, step: T) -> Rule<T> {
    Rule::Range(start, end, step)
}

/// 🧉 » create a `Rule` that will match many values
pub fn many<T: PrimInt>(values: Vec<T>) -> Rule<T> {
    Rule::Many(values)
}

/// 🧉 » create a `Rule` that will match a list of rages
pub fn ranges<T: PrimInt>(ranges: Vec<(T, T, T)>) -> Rule<T> {
    Rule::Ranges(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_val() {
        let rule = val(5);
        assert!(rule.matches(5));
        assert!(!rule.matches(6));
    }

    #[test]
    fn test_rule_range() {
        let rule = range(5, 10, 1);
        assert!(rule.matches(5));
        assert!(rule.matches(6));
        assert!(rule.matches(10));
        assert!(!rule.matches(11));
    }

    #[test]
    fn test_rule_range_step() {
        let rule = range(5, 10, 2);
        assert!(rule.matches(5));
        assert!(!rule.matches(6));
        assert!(rule.matches(7));
        assert!(!rule.matches(8));
        assert!(rule.matches(9));
        assert!(!rule.matches(10));
        assert!(!rule.matches(11));
    }

    #[test]
    fn test_rule_wrapping_range() {
        // imagine a week where 0 is Sunday and 6 is Saturday
        // s | m | t | w | t | f | s
        // 0 | 1 | 2 | 3 | 4 | 5 | 6

        // if a range rules goes from 5 to 2, it should match 5, 6, 0, 1, 2
        // since our rule doesn't have max and min values, it should match any value
        // >= 5 and any value <= 2, (well, taking step into account, obviously)

        // TODO: maybe in the future we could add another rule type "WrappingRange" that would
        //       handle this case more explicitly and taking into account the min-max wrapping
        //       limits (like 0 and 6 in this case). It obviously would need its WrappingRanges
        //       analogs just like we have Ranges for Range... for now we can just use the
        //       Range rule and be happy with it

        let rule = range(5, 2, 1);
        assert!(rule.matches(5));
        assert!(rule.matches(6));
        assert!(rule.matches(0));
        assert!(rule.matches(1));
        assert!(rule.matches(2));
        assert!(!rule.matches(3));
        assert!(!rule.matches(4));

        // would match any value >= 5 too
        assert!(rule.matches(7));
    }

    #[test]
    fn test_rule_many() {
        let rule = many(vec![5, 10, 15]);
        assert!(rule.matches(5));
        assert!(rule.matches(10));
        assert!(rule.matches(15));
        assert!(!rule.matches(11));
    }

    #[test]
    fn test_rule_ranges() {
        let rule = ranges(vec![(5, 10, 1), (15, 20, 1)]);
        assert!(rule.matches(5));
        assert!(rule.matches(6));
        assert!(rule.matches(10));
        assert!(rule.matches(15));
        assert!(rule.matches(20));
        assert!(!rule.matches(11));
    }
}
