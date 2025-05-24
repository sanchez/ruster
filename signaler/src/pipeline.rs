//! Pipeline module provides a composable data transformation pipeline.
//!
//! This module implements a type-safe pipeline pattern that allows for chaining multiple
//! transformations together in a clean and functional way. The pipeline can handle both
//! direct transformations and optional transformations (using `Option<T>`).
//!
//! # Key Features
//!
//! - Type-safe transformations between different data types
//! - Support for optional transformations with `and_then`
//! - Filtering capabilities with custom predicates
//! - Lazy evaluation - transformations only occur when `execute` is called
//!
//! # Example
//!
//! ```rust
//! use signaler::Pipeline;
//!
//! // Create a pipeline that processes numbers
//! let pipeline = Pipeline::new()
//!     .map(|x: i32| x * 2)                  // Double the input
//!     .map(|x| x.to_string())              // Convert to string
//!     .filter(|s| s.parse::<i32>().unwrap() > 5);  // Keep only values > 5
//!
//! assert_eq!(pipeline.execute(3), Some("6".to_string()));
//! assert_eq!(pipeline.execute(1), None);  // Filtered out because 2 <= 5
//! ```
/// A pipeline that transforms data from one type to another through a series of transformations
/// in a type-safe and composable way.
///
pub struct Pipeline<TInput: 'static, TOutput: 'static> {
    transformer: Box<dyn Fn(TInput) -> TOutput>,
}

impl<T> Pipeline<T, T> {
    /// Creates a new pipeline that initially acts as an identity function.
    ///
    /// The new pipeline will simply pass through any input value without modification.
    /// This serves as the starting point for building more complex transformations.
    ///
    /// # Returns
    ///
    /// Returns a new `Pipeline<T, T>` that acts as an identity function.
    ///
    /// # Examples
    ///
    /// ```
    /// use signaler::Pipeline;
    ///
    /// let pipeline = Pipeline::<i32, i32>::new();
    /// assert_eq!(pipeline.execute(42), 42);
    /// ```
    pub fn new() -> Self {
        Pipeline {
            transformer: Box::new(|x| x),
        }
    }
}

impl<TInput, TOutput> Pipeline<TInput, TOutput> {
    /// Executes the pipeline transformation with the provided input value.
    ///
    /// This method applies all the transformations in the pipeline to the input
    /// value and returns the final result.
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform
    ///
    /// # Returns
    ///
    /// Returns the transformed value of type `TOutput`
    ///
    /// # Examples
    ///
    /// ```
    /// use signaler::Pipeline;
    ///
    /// let pipeline = Pipeline::new()
    ///     .map(|x: i32| x.to_string());
    ///
    /// assert_eq!(pipeline.execute(42), "42");
    /// ```
    pub fn execute(&self, input: TInput) -> TOutput {
        (self.transformer)(input)
    }

    /// Adds a new transformation step to the pipeline.
    ///
    /// This method allows you to chain a new transformation function that converts
    /// the current output type to a new type. The transformation is lazy and will
    /// only be applied when `execute` is called.
    ///
    /// # Type Parameters
    ///
    /// * `TNext` - The type that the new transformation will produce
    /// * `F` - The type of the transformation function
    ///
    /// # Parameters
    ///
    /// * `f` - A function that transforms `TOutput` into `TNext`
    ///
    /// # Returns
    ///
    /// Returns a new `Pipeline` that includes the additional transformation
    ///
    /// # Examples
    ///
    /// ```
    /// use signaler::Pipeline;
    ///
    /// let pipeline = Pipeline::new()
    ///     .map(|x: i32| x * 2)
    ///     .map(|x| x.to_string());
    ///
    /// assert_eq!(pipeline.execute(21), "42");
    /// ```
    pub fn map<TNext, F>(self, f: F) -> Pipeline<TInput, TNext>
    where
        F: 'static + Fn(TOutput) -> TNext,
        TNext: 'static,
    {
        Pipeline {
            transformer: Box::new(move |input| {
                let intermediate = (self.transformer)(input);
                f(intermediate)
            }),
        }
    }

    /// Chains a function that returns an Option, enabling optional transformations.
    ///
    /// This method is useful when a transformation might not always be possible or
    /// valid for all inputs. If the function returns None, the pipeline will
    /// short-circuit and return None for all subsequent operations.
    ///
    /// # Type Parameters
    ///
    /// * `TNext` - The type that will be wrapped in `Option` if the transformation succeeds
    /// * `F` - The type of the transformation function
    ///
    /// # Parameters
    ///
    /// * `f` - A function that transforms `TOutput` into `Option<TNext>`
    ///
    /// # Returns
    ///
    /// Returns a new `Pipeline` that produces optional values
    ///
    /// # Examples
    ///
    /// ```
    /// use signaler::Pipeline;
    ///
    /// let pipeline = Pipeline::new()
    ///     .map(|x: i32| x.to_string())
    ///     .and_then(|s| if s.len() > 1 { Some(s + "!") } else { None });
    ///
    /// assert_eq!(pipeline.execute(42), Some("42!".to_string()));
    /// assert_eq!(pipeline.execute(5), None);
    /// ```
    pub fn and_then<TNext, F>(self, f: F) -> Pipeline<TInput, Option<TNext>>
    where
        F: 'static + Fn(TOutput) -> Option<TNext>,
        TNext: 'static,
    {
        Pipeline {
            transformer: Box::new(move |input| {
                let intermediate = (self.transformer)(input);
                f(intermediate)
            }),
        }
    }

    /// Adds a filter predicate to the pipeline, converting it to handle Options.
    ///
    /// This method transforms the pipeline to produce `Option<TOutput>`, where the
    /// value becomes `None` if it doesn't satisfy the predicate.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The type of the predicate function
    ///
    /// # Parameters
    ///
    /// * `predicate` - A function that takes a reference to `TOutput` and returns a boolean
    ///
    /// # Returns
    ///
    /// Returns a new `Pipeline` that produces optional values based on the predicate
    ///
    /// # Examples
    ///
    /// ```
    /// use signaler::Pipeline;
    ///
    /// let pipeline = Pipeline::new()
    ///     .map(|x: i32| x * 2)
    ///     .filter(|&x| x > 5);
    ///
    /// assert_eq!(pipeline.execute(3), Some(6));
    /// assert_eq!(pipeline.execute(1), None);  // Filtered out because 2 <= 5
    /// ```
    pub fn filter<F>(self, predicate: F) -> Pipeline<TInput, Option<TOutput>>
    where
        F: 'static + Fn(&TOutput) -> bool,
    {
        Pipeline {
            transformer: Box::new(move |input| {
                let result = (self.transformer)(input);
                if predicate(&result) {
                    Some(result)
                } else {
                    None
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pipeline_identity() {
        let pipeline = Pipeline::<i32, i32>::new();
        assert_eq!(pipeline.execute(42), 42);
        assert_eq!(pipeline.execute(-1), -1);
    }

    #[test]
    fn test_map_single_transformation() {
        let pipeline = Pipeline::new().map(|x: i32| x.to_string());
        assert_eq!(pipeline.execute(42), "42");
    }

    #[test]
    fn test_map_multiple_transformations() {
        let pipeline = Pipeline::new()
            .map(|x: i32| x * 2)
            .map(|x| x.to_string())
            .map(|s| s + "!");
        assert_eq!(pipeline.execute(21), "42!");
    }

    #[test]
    fn test_filter_basic() {
        let pipeline = Pipeline::new().map(|x: i32| x * 2).filter(|&x| x > 5);

        assert_eq!(pipeline.execute(3), Some(6));
        assert_eq!(pipeline.execute(1), None);
    }

    #[test]
    fn test_and_then_conversion() {
        let pipeline = Pipeline::new().map(|x: i32| x * 2).and_then(|x| {
            if x > 5 {
                Some(x.to_string())
            } else {
                None
            }
        });

        assert_eq!(pipeline.execute(3), Some("6".to_string()));
        assert_eq!(pipeline.execute(1), None);
    }
    #[test]
    fn test_pipeline_composition() {
        let filtered_pipeline = Pipeline::new().map(|x: i32| x * 2).filter(|&x| x > 5);

        let string_pipeline = Pipeline::new().map(|x: i32| x * 2).map(|x| x.to_string());

        assert_eq!(filtered_pipeline.execute(3), Some(6));
        assert_eq!(filtered_pipeline.execute(1), None);
        assert_eq!(string_pipeline.execute(21), "42");
    }

    #[test]
    fn test_pipeline_with_different_types() {
        let pipeline = Pipeline::new()
            .map(|x: i32| x.to_string())
            .map(|s| s.len())
            .map(|len| len > 1);

        assert_eq!(pipeline.execute(42), true);
        assert_eq!(pipeline.execute(5), false);
    }
}
