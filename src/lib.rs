//!
//! The RandomSelector selects among weighted choices, without bias.
//!
//! ```
//! use rand_select::RandomSelector;
//! let selector = RandomSelector::default()
//!    .with(1.0, 'A')
//!    .with(1.5, 'B')
//!    .with_none(3.0);
//! let l = selector.select();
//! // l has half a chance to be None, and is 50% more likely to be 'B' than 'A'
//! ```
//!
//! If you set a value and call neither `with_none` nor `with_none_up_to`, the selector will always return a value.
//!
//! If you have already normalized weight, `with_none_up_to` is a convenient way to set the total weight of the selector:
//!
//! ```
//! use rand_select::RandomSelector;
//! let selector = RandomSelector::default()
//!    .with(0.1, 'A')
//!    .with(0.2, 'B')
//!    .with_none_up_to(1.0);
//! ```
//! The RandomSelector is designed for reuse, and can use the RNG of your choice.

use {
    rand::Rng,
};

#[derive(Clone)]
struct Choice<T> {
    weight: f64,
    value: T,
}

/// A selector allowing to randomly select a value from a set of choices, each with an associated weight.
///
/// ```
/// use rand_select::RandomSelector;
/// let selector = RandomSelector::default()
///    .with(1.0, 'A')
///    .with(1.5, 'B')
///    .with_none(3.0);
/// let l = selector.select();
/// // l has half a chance to be None, and is 50% more likely to be 'B' than 'A'
/// ```
#[derive(Clone)]
pub struct RandomSelector<T> {
    choices: Vec<Choice<T>>,
    total_weight: f64,
}

impl<T> Default for RandomSelector<T> {
    fn default() -> Self {
        Self {
            choices: Vec::new(),
            total_weight: 0.0,
        }
    }
}

impl<T> RandomSelector<T> {
    pub fn with(mut self, weight: f64, value: T) -> Self {
        self.choices.push(Choice { weight, value });
        self.total_weight += weight.abs();
        self
    }
    pub fn with_none(mut self, weight: f64) -> Self {
        self.total_weight += weight.abs();
        self
    }
    /// Complete choices to be None up to the given weight.
    ///
    /// This is convenient where all choices set are conventionnaly already normalized:
    ///
    /// ```
    /// use rand_select::RandomSelector;
    /// let selector = RandomSelector::default()
    ///    .with(0.1, 'A')
    ///    .with(0.2, 'B')
    ///    .with_none_up_to(1.0);
    /// ```
    pub fn with_none_up_to(mut self, total_weight: f64) -> Self {
        self.total_weight = total_weight.abs();
        self
    }
    /// Select a random value among the provided ones.
    pub fn select(&self) -> Option<&T> {
        let mut rng = rand::rng();
        self.select_with_rng(&mut rng)
    }
    /// Select a random value among the provided ones, with the generator of your choice.
    pub fn select_with_rng<R: Rng>(&self, mut r: R) -> Option<&T> {
        if self.total_weight == 0.0 {
            return None;
        }
        let random_value: f64 = r.random_range(0.0..self.total_weight);
        let mut cumulative_weight = 0.0;
        for choice in &self.choices {
            cumulative_weight += choice.weight;
            if random_value < cumulative_weight {
                return Some(&choice.value);
            }
        }
        None
    }
}
