//! Provides a generic proxy wrapper that allows attaching arbitrary properties to any type.
//!
//! The `ProxyObject` type serves as a wrapper around any type `T`, providing additional
//! property storage capabilities while maintaining transparent access to the wrapped object
//! through deref coercion.
//!
//! # Examples
//!
//! ```
//! use dater::{ProxyObject, PropertyValue};
//!
//! // Create a proxy using the constructor
//! let mut proxy = ProxyObject::new(String::from("Hello"));
//! proxy.set("language", PropertyValue::String("en".to_string()));
//!
//! // Access the wrapped object's methods directly
//! assert_eq!(proxy.len(), 5);
//!
//! // Access custom properties
//! assert_eq!(proxy.get("language").unwrap().to_string(), "en");
//!
//! // Create a proxy using Into trait
//! let proxy: ProxyObject<_> = "World".to_string().into();
//! assert_eq!(proxy.len(), 5);
//! ```

use std::ops::Deref;

use crate::{PropertyCollection, PropertyValue};

/// A proxy wrapper that adds property storage capabilities to any type.
///
/// This type acts as a transparent wrapper around the contained type `T` while
/// providing additional key-value property storage. The wrapped object can be
/// accessed through deref coercion, making the proxy behave like the original type.
pub struct ProxyObject<T: ?Sized> {
    /// The wrapped object that this proxy provides access to
    object: Box<T>,
    /// Collection of additional properties associated with the object
    properties: PropertyCollection,
}

impl<T: ?Sized> Deref for ProxyObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.object
    }
}

impl<T> From<T> for ProxyObject<T> {
    /// Creates a new `ProxyObject` from the given value.
    ///
    /// This implementation allows for convenient conversion of any type into a `ProxyObject`
    /// using `Into::into()`.
    ///
    /// # Examples
    ///    /// ```
    /// use dater::ProxyObject;
    ///
    /// let value = 42;
    /// let proxy: ProxyObject<_> = value.into();
    /// assert_eq!(*proxy, 42);
    ///
    /// // Works with any type
    /// let string_proxy: ProxyObject<String> = "Hello".to_string().into();
    /// assert_eq!(&*string_proxy, "Hello");
    /// ```
    fn from(object: T) -> Self {
        ProxyObject::new(object)
    }
}

impl<T> ProxyObject<T> {
    /// Creates a new `ProxyObject` wrapping the provided object.
    ///
    /// The new proxy starts with an empty property collection. Properties can be
    /// added later using the `set` method.
    ///
    /// # Examples
    ///    /// ```
    /// use dater::ProxyObject;
    ///
    /// let proxy = ProxyObject::new(42);
    /// assert_eq!(*proxy, 42);
    /// ```
    pub fn new(object: T) -> Self {
        ProxyObject {
            object: Box::new(object),
            properties: PropertyCollection::new(),
        }
    }

    /// Associates a property value with the given key.
    ///
    /// If a property with the same key already exists, it will be replaced.
    ///
    /// # Examples
    ///    /// ```
    /// use dater::{ProxyObject, PropertyValue};
    ///
    /// let mut proxy = ProxyObject::new(42);
    /// proxy.set("answer", PropertyValue::Integer(42));
    /// ```
    pub fn set(&mut self, key: &str, value: PropertyValue) {
        self.properties.set(key, value);
    }

    /// Retrieves the property value associated with the given key.
    ///
    /// Returns `None` if the property doesn't exist.
    ///
    /// # Examples
    ///    /// ```
    /// use dater::{ProxyObject, PropertyValue};
    ///
    /// let mut proxy = ProxyObject::new(42);
    /// proxy.set("answer", PropertyValue::Integer(42));
    /// assert_eq!(proxy.get("answer").unwrap().to_string(), "42");
    /// assert_eq!(proxy.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_object_creation() {
        let proxy = ProxyObject::new(42);
        assert_eq!(*proxy, 42);
    }

    #[test]
    fn test_proxy_deref() {
        let string = String::from("test");
        let proxy = ProxyObject::new(string);
        assert_eq!(proxy.len(), 4);
        assert_eq!(proxy.as_str(), "test");
    }
    #[test]
    fn test_property_access() {
        let mut proxy = ProxyObject::new(42);

        // Test setting and getting an integer property
        proxy.set("int_key", PropertyValue::Integer(123));
        if let PropertyValue::Integer(val) = proxy.get("int_key").unwrap() {
            assert_eq!(*val, 123);
        } else {
            panic!("Expected Integer property value");
        }

        // Test non-existent property
        assert!(proxy.get("nonexistent").is_none());

        // Test property overwrite
        proxy.set("key", PropertyValue::String("value".to_string()));
        if let PropertyValue::String(val) = proxy.get("key").unwrap() {
            assert_eq!(val, "value");
        } else {
            panic!("Expected String property value");
        }

        // Test different property types
        proxy.set("float", PropertyValue::Float(3.14));
        proxy.set("bool", PropertyValue::Boolean(true));
        proxy.set(
            "date",
            PropertyValue::DateTime("2025-05-24T00:00:00Z".to_string()),
        );

        if let PropertyValue::Float(val) = proxy.get("float").unwrap() {
            assert_eq!(*val, 3.14);
        }
        if let PropertyValue::Boolean(val) = proxy.get("bool").unwrap() {
            assert!(val);
        }
        if let PropertyValue::DateTime(val) = proxy.get("date").unwrap() {
            assert_eq!(val, "2025-05-24T00:00:00Z");
        }
    }

    #[test]
    fn test_from_implementation() {
        let value = 42;
        let proxy: ProxyObject<i32> = value.into();
        assert_eq!(*proxy, 42);
    }
    #[test]
    fn test_with_unsized_type() {
        let string: Box<str> = Box::from("test");
        let proxy: ProxyObject<str> = ProxyObject {
            object: string,
            properties: PropertyCollection::new(),
        };
        assert_eq!(&*proxy, "test");
    }
}
