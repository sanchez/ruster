use std::{collections::HashMap, fmt, ops::Index};

/// Represents different types of values that can be stored in a property collection.
/// This enum provides type safety while allowing flexibility in the types of data
/// that can be managed within the collection.
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    /// String values for text-based properties
    String(String),
    /// Integer values for whole number properties
    Integer(i64),
    /// Float values for decimal number properties
    Float(f64),
    /// Boolean values for true/false properties
    Boolean(bool),
    /// DateTime values stored in ISO 8601 format for temporal properties
    DateTime(String),
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyValue::String(s) => write!(f, "{}", s),
            PropertyValue::Integer(i) => write!(f, "{}", i),
            PropertyValue::Float(fl) => write!(f, "{}", fl),
            PropertyValue::Boolean(b) => write!(f, "{}", b),
            PropertyValue::DateTime(dt) => write!(f, "{}", dt),
        }
    }
}

/// A flexible key-value store for managing properties of different types.
///
/// `PropertyCollection` provides a type-safe way to store and retrieve different
/// kinds of values, making it suitable for configuration management, metadata
/// storage, or any scenario requiring dynamic property handling.
///
/// # Example
///
/// ```
/// use dater::{PropertyCollection, PropertyValue};
///
/// let mut props = PropertyCollection::new();
/// props.set("name", PropertyValue::String("example".to_string()));
/// props.set("count", PropertyValue::Integer(42));
///
/// assert!(matches!(props.get("name"), Some(PropertyValue::String(_))));
/// ```
#[derive(Default, Debug)]
pub struct PropertyCollection {
    props: HashMap<String, PropertyValue>,
}

impl PropertyCollection {
    /// Creates a new, empty property collection.
    ///
    /// Returns a `PropertyCollection` instance ready to store properties.
    pub fn new() -> Self {
        PropertyCollection {
            props: HashMap::new(),
        }
    }

    /// Sets a property value for the given key.
    ///
    /// If the key already exists, its value will be updated with the new value.
    ///
    /// # Parameters
    ///
    /// * `key` - The identifier for the property
    /// * `value` - The value to store for the property
    pub fn set(&mut self, key: &str, value: PropertyValue) {
        self.props.insert(key.to_string(), value);
    }

    /// Retrieves the value associated with the given key.
    ///
    /// Returns `None` if the key doesn't exist in the collection.
    ///
    /// # Parameters
    ///
    /// * `key` - The identifier of the property to retrieve
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.props.get(key)
    }

    /// Removes a property from the collection.
    ///
    /// If the key doesn't exist, this operation has no effect.
    ///
    /// # Parameters
    ///
    /// * `key` - The identifier of the property to remove
    pub fn remove(&mut self, key: &str) {
        self.props.remove(key);
    }

    /// Removes all properties from the collection.
    pub fn clear(&mut self) {
        self.props.clear();
    }

    /// Returns the number of properties in the collection.
    pub fn len(&self) -> usize {
        self.props.len()
    }

    /// Returns true if the collection contains no properties.
    pub fn is_empty(&self) -> bool {
        self.props.is_empty()
    }
}

impl Index<&str> for PropertyCollection {
    type Output = PropertyValue;

    /// Provides index-based access to property values.
    ///
    /// # Panics
    ///
    /// Will panic if the key doesn't exist in the collection. Use `get()`
    /// instead if you want to handle missing keys gracefully.
    fn index(&self, key: &str) -> &Self::Output {
        self.props.get(key).expect("No property found for key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_collection_is_empty() {
        // Arrange & Act
        let collection = PropertyCollection::new();

        // Assert
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        // Arrange
        let mut collection = PropertyCollection::new();
        let test_key = "test_key";
        let test_value = PropertyValue::String("test_value".to_string());

        // Act
        collection.set(test_key, test_value.clone());

        // Assert
        assert_eq!(collection.get(test_key), Some(&test_value));
    }

    #[test]
    fn test_remove() {
        // Arrange
        let mut collection = PropertyCollection::new();
        let test_key = "test_key";
        collection.set(test_key, PropertyValue::Integer(42));

        // Act
        collection.remove(test_key);

        // Assert
        assert_eq!(collection.get(test_key), None);
    }

    #[test]
    fn test_clear() {
        // Arrange
        let mut collection = PropertyCollection::new();
        collection.set("key1", PropertyValue::Integer(1));
        collection.set("key2", PropertyValue::String("value2".to_string()));

        // Act
        collection.clear();

        // Assert
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_index_operator() {
        // Arrange
        let mut collection = PropertyCollection::new();
        let test_key = "test_key";
        let test_value = PropertyValue::Boolean(true);
        collection.set(test_key, test_value.clone());

        // Act & Assert
        assert_eq!(&collection[test_key], &test_value);
    }

    #[test]
    #[should_panic(expected = "No property found for key")]
    fn test_index_operator_missing_key() {
        // Arrange
        let collection = PropertyCollection::new();

        // Act - This should panic
        let _value = &collection["nonexistent"];
    }

    #[test]
    fn test_different_property_types() {
        // Arrange
        let mut collection = PropertyCollection::new();

        // Act
        collection.set("string", PropertyValue::String("text".to_string()));
        collection.set("integer", PropertyValue::Integer(42));
        collection.set("float", PropertyValue::Float(3.14));
        collection.set("bool", PropertyValue::Boolean(true));
        collection.set(
            "date",
            PropertyValue::DateTime("2025-05-24T12:00:00Z".to_string()),
        );

        // Assert
        assert!(matches!(
            collection.get("string"),
            Some(PropertyValue::String(_))
        ));
        assert!(matches!(
            collection.get("integer"),
            Some(PropertyValue::Integer(_))
        ));
        assert!(matches!(
            collection.get("float"),
            Some(PropertyValue::Float(_))
        ));
        assert!(matches!(
            collection.get("bool"),
            Some(PropertyValue::Boolean(_))
        ));
        assert!(matches!(
            collection.get("date"),
            Some(PropertyValue::DateTime(_))
        ));
    }
}
