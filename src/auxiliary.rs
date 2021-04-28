pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub const LATIN25: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
pub const LATIN26: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LATIN36: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
