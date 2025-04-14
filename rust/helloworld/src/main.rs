use std::env;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rand:: {
    rng,
    prelude::IndexedRandom
};

// Hashmap of the values of Hello, World!
static GREETING: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("en", "Hello, world!"),
        ("el", "Γειά σου, κόσμε!"),
        ("gd", "Halò, a shaoghail!"),
        ("mk", "Здраво, свету!"),
        ("he", "שלום, עולם!"),
        ("fr", "Bonjour, le monde!"),
    ])
});

/*  If no language is given as argument,
    a random language is taken from key list
    and shown the greetings for that language
*/

fn get_random_key() -> &'static str {
    let keys: Vec<_> = GREETING.keys().copied().collect();
    let mut rng = rng();

    keys.choose(&mut rng).copied().unwrap_or("en")
}

fn greet(lang: Option<String>) -> String {
    let code = match lang {
        Some(l) if !l.is_empty() => l,
        _ => get_random_key().to_string(),
    };

    GREETING
        .get(code.as_str())
        .unwrap_or(&"Unknown language code.")
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let lang = args.get(0).cloned();
    println!("{}", greet(lang));
}


/* Tests for the file */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_known_languages() {
        assert_eq!(greet(Some("en".to_string())), "Hello, world!");
        assert_eq!(greet(Some("fr".to_string())), "Bonjour, le monde!");
        assert_eq!(greet(Some("he".to_string())), "שלום, עולם!");
    }

    #[test]
    fn test_greet_unknown_language() {
        assert_eq!(greet(Some("zz".to_string())), "Unknown language code.");
    }

    #[test]
    fn test_greet_empty_or_none_defaults_to_valid_key() {
        let result = greet(None);
        assert!(
            GREETING.values().any(|&val| val == result),
            "Greeting should be one of the valid greetings"
        );

        let result = greet(Some("".to_string()));
        assert!(
            GREETING.values().any(|&val| val == result),
            "Greeting should be one of the valid greetings"
        );
    }

    #[test]
    fn test_random_key_returns_valid_key() {
        let key = get_random_key();
        assert!(
            GREETING.contains_key(key),
            "Random key should be from the GREETING keys"
        );
    }
}