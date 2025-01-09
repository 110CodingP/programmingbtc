pub trait Serializer {
    fn sec(&self, is_compressed: bool) -> String;
}