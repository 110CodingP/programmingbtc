pub trait Serializer {
    fn sec(&self) -> Vec<u8>;
}