fn banner(message: &str) -> String {
    format!("*** {} ***", message.to_uppercase())
}

fn main() {
    println!("{}", banner("larry strat"));
}

#[cfg(test)]
mod tests {
    use super::banner;

    #[test]
    fn banner_wraps_text() {
        assert_eq!(banner("demo"), "*** DEMO ***");
    }
}
