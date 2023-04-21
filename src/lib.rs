pub fn details() -> &'static str {
    if cfg!(feature = "upper") {
        let upper = String::from("I am a remote module.").to_uppercase();
        println!("{upper}");
        "UPPER"
    } else if cfg!(feature = "lower") {
        println!("I am a remote module.");
        "lower"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::details;

    #[cfg(feature = "upper")]
    #[test]
    fn test_details_upper() {
        assert_eq!(details(), "UPPER");
    }

    #[cfg(feature = "lower")]
    #[test]
    fn test_details_lower() {
        assert_eq!(details(), "lower");
    }
    #[cfg(not(any(feature = "upper", feature = "lower")))]
    #[test]
    fn test_details_none() {
        assert_eq!(details(), "");
    }
}
