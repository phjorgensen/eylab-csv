use std::env::args;

pub fn get_value_from(argument_name: &str, expect_string: &str) -> String {
    return args()
        .position(|arg| arg == format!("--{}", argument_name))
        .and_then(|idx| args().nth(idx + 1))
        .expect(expect_string);
}

pub fn has_value(argument_name: &str) -> bool {
    return args()
        .find(|arg| arg.eq(argument_name))
        .and_then(|_| Some(true))
        .unwrap_or(false);
}

pub fn nth(idx: usize, expect_string: &str) -> String {
    return args().nth(idx).expect(expect_string);
}

