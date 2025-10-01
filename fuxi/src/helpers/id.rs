#[inline]
pub fn new() -> String {
    uuid::Uuid::now_v7().simple().to_string()
}
