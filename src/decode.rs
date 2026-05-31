use std::time::{SystemTime};
use chrono::{DateTime, Local};

pub fn format_date(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    format!("{}", datetime.format("%b %d %H:%M"))
}


pub fn format_permissions(mode: u32) -> String {
    let file_type = if mode & 0o40000 == 0o40000 { 'd' } else { '-' };

    let rwx = |bits| {
        let r = if bits & 0b100 != 0 { 'r' } else { '-' };
        let w = if bits & 0b010 != 0 { 'w' } else { '-' };
        let x = if bits & 0b001 != 0 { 'x' } else { '-' };
        format!("{}{}{}", r, w, x)
    };

    let owner = rwx((mode >> 6) & 0b111);
    let group = rwx((mode >> 3) & 0b111);
    let other = rwx(mode & 0b111);

    format!("{}{}{}{}", file_type, owner, group, other)
}
