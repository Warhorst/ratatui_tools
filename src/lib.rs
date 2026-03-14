use smallvec::{SmallVec, smallvec};

pub mod cells;
pub mod tilemap;

/// A helper struct for a message driven workflow. It can capture multiple
/// messages from type T, which are created from multiple message producers (like sub widgets).
/// They can then be processed by a message consumer (like the app updating the app state).
pub struct Messages<T>(SmallVec<[T; 4]>);

impl<T> Messages<T> {
    /// Create an empty [Messages] struct.
    pub fn none() -> Self {
        Messages(smallvec![])
    }

    /// Create a [Messages] instance with exactly one message.
    pub fn single(message: T) -> Self {
        Messages(smallvec![message])
    }

    /// Create a [Messages] instance containing all the given messages.
    pub fn multi(messages: impl IntoIterator<Item = T>) -> Self {
        Messages(messages.into_iter().collect())
    }

    /// Create a [Messages] instance from multiple other [Messages], combining all their
    /// messages from type [T] into one.
    pub fn from_messages(messages: impl IntoIterator<Item = Messages<T>>) -> Self {
        Messages(messages.into_iter().flat_map(|ms| ms.into_iter()).collect())
    }

    /// Return an [Iterator] with references to all messages of type [T].
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T> IntoIterator for Messages<T> {
    type Item = T;

    type IntoIter = smallvec::IntoIter<[T; 4]>;

    /// Consume this [Messages] instance and return all its messages from type [T].
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
