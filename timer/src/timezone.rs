#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TimeZone {
    // pub name: String, // e.g., "America/New_York"
    pub offset: i32, // Offset in seconds from UTC
}
