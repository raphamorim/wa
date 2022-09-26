use std::collections::HashMap;

mod utils;

/// (String) replace extended ASCII codes to standards.
///
/// # Example
///
/// ```rust
/// use wa::string::replace_extended_ascii;
///
/// assert_eq!(replace_extended_ascii("São Paulo".to_string()), "Sao Paulo");
/// assert_eq!(replace_extended_ascii("Água".to_string()), "Agua");
/// assert_eq!(replace_extended_ascii("Pão".to_string()), "Pao");
/// assert_eq!(replace_extended_ascii("Åke".to_string()), "Ake");
/// assert_eq!(replace_extended_ascii("Södermalm".to_string()), "Sodermalm");
/// assert_eq!(replace_extended_ascii("Rio de Janeiro".to_string()), "Rio de Janeiro");
/// ```
///

#[no_mangle]
pub fn replace_extended_ascii(s: String) -> String {
    let mut ns = String::new();
    let replace_map = utils::get_hashmap_to_replace_extended();

    for (_, c) in s.chars().enumerate() {
        if replace_map.contains_key(&c) {
            let nc = replace_map.get(&c).unwrap();
            ns.push(*nc);
        } else {
            ns.push(c);
        }
    }

    ns
}

#[test]
fn test_replace_extended_ascii() {
    assert_eq!(replace_extended_ascii("São Paulo".to_string()), "Sao Paulo");
    assert_eq!(replace_extended_ascii("Água".to_string()), "Agua");
    assert_eq!(replace_extended_ascii("Pão".to_string()), "Pao");
    assert_eq!(replace_extended_ascii("Åke".to_string()), "Ake");
    assert_eq!(replace_extended_ascii("Södermalm".to_string()), "Sodermalm");
    assert_eq!(
        replace_extended_ascii("Rio de Janeiro".to_string()),
        "Rio de Janeiro"
    );
}

/// (String) Creates a compiled template function that can interpolate data properties in "interpolate" delimiters
///
/// Note: Is necessary spaces between configs.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use wa::string::template;
///
/// let url_template = template("https://api.com/{ user_id }/products/{ product_id }".to_string());
/// let url: String = url_template(HashMap::from([
///     ("user_id", "85"),
///     ("product_id", "23"),
/// ]));
///
/// ```
///

#[no_mangle]
pub fn template(s: String) -> impl Fn(HashMap<&str, &str>) -> String {
    let compile_template_fn = move |param: HashMap<&str, &str>| compile_template(&s, param);
    compile_template_fn
}

fn compile_template(template_string: &str, config: HashMap<&str, &str>) -> String {
    let mut ct = template_string.to_string();

    for (key, value) in &config {
        let k = format!("{{ {} }}", key);
        ct = ct.replace(&k, value);
    }
    ct
}

#[test]
fn test_template() {
    let url_template = template("https://api.com/{ user_id }/products/{ product_id }".to_string());

    let url: String = url_template(HashMap::from([("user_id", "85"), ("product_id", "23")]));
    let url_2: String = url_template(HashMap::from([("user_id", "23"), ("product_id", "85")]));

    assert_eq!(url, "https://api.com/85/products/23");
    assert_eq!(url_2, "https://api.com/23/products/85");
}

/// (String) Creates a compiled template function that can interpolate data properties in "interpolate" delimiters using vector
///
/// # Example
///
/// ```rust
/// use wa::string::template_vec;
///
/// let url_template = template_vec("https://api.com/{1}/products/{2}?by={1}".to_string());
///
/// let url: String = url_template(vec!["85", "23"]); // "https://api.com/85/products/23?by=85"
/// let url_2: String = url_template(vec!["23", "85"]); // "https://api.com/23/products/85?by=23"
/// ```
///

#[no_mangle]
pub fn template_vec(s: String) -> impl Fn(Vec<&str>) -> String {
    let compile_template_fn = move |param: Vec<&str>| compile_template_vec(&s, param);
    compile_template_fn
}

fn compile_template_vec(template_string: &str, config: Vec<&str>) -> String {
    let mut ct = template_string.to_string();

    for (i, value) in config.iter().enumerate() {
        let k = format!("{{{}}}", i + 1);
        ct = ct.replace(&k, value);
    }
    ct
}

#[test]
fn test_template_vec() {
    let url_template = template_vec("https://api.com/{1}/products/{2}?by={1}".to_string());

    let url: String = url_template(vec!["85", "23"]);
    let url_2: String = url_template(vec!["23", "85"]);

    assert_eq!(url, "https://api.com/85/products/23?by=85");
    assert_eq!(url_2, "https://api.com/23/products/85?by=23");
}

/// (String) Converts the input string into Camel-Case format.
///
/// Note: All special characters will be removed and only valid letters remained.
///
/// To replace extended ASCII codes to standards, e.g: "ã" to "a". Check `wa::string::replace_extended_ascii`.
///
/// # Example
///
/// ```rust
/// use wa::string::camel_case;
///
/// camel_case("São Paulo".to_string()); // => "sãoPaulo"
///
/// camel_case("____Rio____de___JANEIRO".to_string()); // => "rioDeJaneiro"
///
/// camel_case("Rio de Janeiro".to_string()); // => "rioDeJaneiro"
///
/// camel_case("--stock-----holm--".to_string()); // => "stockHolm"
///
/// camel_case("--stock-----holm--".to_string()); // => "stockHolm"
///
/// camel_case("Rio2DE2janeiro".to_string()); // => "rio2de2janeiro"
/// ```
///

#[no_mangle]
pub fn camel_case(s: String) -> String {
    let mut cc = String::new();
    let mut is_special = false;

    for (_, character) in s.chars().enumerate() {
        if !character.is_alphanumeric() {
            is_special = true;
            continue;
        }

        if is_special && cc.len() > 0 {
            cc.push(character.to_ascii_uppercase());
        } else {
            cc.push(character.to_ascii_lowercase());
        }

        if is_special {
            is_special = false;
        }
    }

    cc
}

#[test]
fn test_camel_case() {
    assert_eq!(camel_case("São Paulo".to_string()), "sãoPaulo");
    assert_eq!(camel_case("Rio de Janeiro".to_string()), "rioDeJaneiro");
    assert_eq!(camel_case("rio, de Janeiro".to_string()), "rioDeJaneiro");
    assert_eq!(camel_case("rio".to_string()), "rio");
    assert_eq!(camel_case("--stock-----holm--".to_string()), "stockHolm");
    assert_eq!(
        camel_case("____Rio____de___JANEIRO".to_string()),
        "rioDeJaneiro"
    );
    assert_eq!(camel_case("Rio2DE2janeiro".to_string()), "rio2de2janeiro");
}

/// (String) Converts the input string into Kebab-Case format.
///
/// Note: All the special characters will be removed and only valid letters remained.
///
/// To replace extended ASCII codes to standards, e.g: "ã" to "a". Check `wa::string::replace_extended_ascii`.
///
/// # Example
///
/// ```rust
/// use wa::string::kebab_case;
///
/// kebab_case("São Paulo".to_string()); // => "são-paulo"
///
/// kebab_case("Rio de Janeiro".to_string()); // => "foo-bar"
///
/// kebab_case("--rio--1--".to_string()); // => "foo-bar"
///
/// kebab_case("__rIO_-de-Jan_eiro_".to_string()); // => "f-oo-ba-r"
/// ```
///

#[no_mangle]
pub fn kebab_case(s: String) -> String {
    let mut kb = String::new();
    let mut is_last_special_char = false;
    let mut is_last_small_char = false;

    for (_, char) in s.chars().enumerate() {
        if char.is_alphanumeric() {
            if is_last_special_char {
                if kb.len() > 0 {
                    kb.push('-');
                }

                is_last_special_char = false;
            } else if is_last_small_char && char.is_uppercase() && kb.len() > 0 {
                kb.push('-');
            }

            kb.push(char.to_ascii_lowercase());

            if char.is_lowercase() {
                is_last_small_char = true;
            } else {
                is_last_small_char = false;
            }
        } else {
            is_last_special_char = true;
        }
    }

    kb
}

#[test]
fn test_kebab_case() {
    assert_eq!(kebab_case("São Paulo".to_string()), "são-paulo");
    assert_eq!(kebab_case("Rio de Janeiro".to_string()), "rio-de-janeiro");
    assert_eq!(
        kebab_case("--rio--1--2-de-3janeiro".to_string()),
        "rio-1-2-de-3janeiro"
    );
    assert_eq!(
        kebab_case("__rIO_-de-Jan_eiro_".to_string()),
        "r-io-de-jan-eiro"
    );
}
