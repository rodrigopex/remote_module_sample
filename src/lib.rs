pub fn details() -> &'static str {
    if cfg!(feature = "upper") {
        let upper = String::from("I am a remote module.").to_uppercase();
        println!("{upper}");
        "UPPERv2"
    } else if cfg!(feature = "lower") {
        println!("I am a remote module.");
        "lowerv2"
    } else {
        "v2"
    }
}

#[cfg(test)]
mod tests {
    use super::details;

    #[cfg(feature = "upper")]
    #[test]
    fn test_details_upper() {
        assert_eq!(details(), "UPPERv2");
    }

    #[cfg(feature = "lower")]
    #[test]
    fn test_details_lower() {
        assert_eq!(details(), "lowerv2");
    }
    #[cfg(not(any(feature = "upper", feature = "lower")))]
    #[test]
    fn test_details_none() {
        assert_eq!(details(), "v2");
    }
}
