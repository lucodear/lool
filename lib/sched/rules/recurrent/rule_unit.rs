use num_traits::PrimInt;

/// 🧉 » a recurrence rule unit
///
/// represents a single rule unit that can be used to match a value
#[derive(Clone)]
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
                if *step == T::one() {
                    value >= *start && value <= *end
                } else {
                    let mut current = *start;
                    while current <= *end {
                        if current == value {
                            return true;
                        }
                        // Move to the next value based on the step size
                        current = current + *step;
                    }
                    false
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
