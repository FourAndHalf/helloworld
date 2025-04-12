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

fn greet(lang: Option<String>) {
    let code = match lang {
        Some(l) if !l.is_empty() => l,
        _ => get_random_key().to_string(),
    };

    match GREETING.get(code.as_str()) {
        Some(message) => println!("{}", message),
        None => println!("Unknown language code."),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let lang = args.get(0).cloned();
    greet(lang);
}