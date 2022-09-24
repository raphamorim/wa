/// (String) Converts the input string into Camel-Case format.
///
/// Note that all the special characters will be removed and only valid letters remained.
///
/// # Example Rust
///
/// ```rust
/// use wa::string::camel_case;
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
/// # Example WebAssembly
///
/// ```javascript
/// use wa::string::camel_case;
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

#[inline]
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
    assert_eq!(camel_case("Rio de Janeiro".to_string()), "rioDeJaneiro");
    assert_eq!(camel_case("rio, de Janeiro".to_string()), "rioDeJaneiro");
    assert_eq!(camel_case("rio".to_string()), "rio");
    assert_eq!(camel_case("--stock-----holm--".to_string()), "stockHolm");
    assert_eq!(camel_case("____Rio____de___JANEIRO".to_string()), "rioDeJaneiro");
    assert_eq!(camel_case("Rio2DE2janeiro".to_string()), "rio2de2janeiro");
}

/// (String) Checks if string ends with the given target string.
///
/// If the position to search up to is not provided, it will search through the whole string by default.
///
/// # Example
///
/// ```rust
/// use wa::string::ends_with;
///
/// let is_ends_with = ends_with("abc".to_string(), "c".to_string(), None);
/// // => true
///
/// let is_ends_with = ends_with("abc".to_string(), "b".to_string(), None);
/// // => false
///
/// let is_ends_with = ends_with("abc".to_string(), "bc".to_string(), Some(2));
/// // => false
/// ```
///

#[inline]
#[no_mangle]
pub fn ends_with(s: String, target: String, position: Option<i32>) -> bool {
  let pos: usize = position.unwrap_or(0) as usize;

  if pos > 0 {
    let sliced = &s[0..pos];
    sliced.ends_with(target.as_str())
  } else {
    s.ends_with(target.as_str())
  }
}

#[test]
fn test_chunk() {
    let is_ends_with = ends_with("abc".to_string(), "c".to_string(), None);
    assert!(is_ends_with, "{}", true);

    // let is_ends_with_2 = ends_with("abc".to_string(), "b".to_string(), None);
    // assert!(is_ends_with_2, "{}", false);
    
    // let is_ends_with_3 = ends_with("abc".to_string(), "bc".to_string(), Some(2));
    // assert!(is_ends_with_3, "{}", false);
}


/// (String) Converts the input string into Kebab-Case format.
///
/// Note that all the special characters will be removed and only valid letters remained.
///
/// # Example
///
/// ```rust
/// use lodust::kebab_case;
///
/// let kebab_cased = kebab_case("Foo Bar".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("--foo--bar--".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("__FOO_BAR__".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("fooBar".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("__fOo_-BaR__".to_string());
/// // => "f-oo-ba-r"
/// ```
///

pub fn kebab_case(s: String) -> String {
  let mut result = String::new();
  let mut is_last_special_char = false;
  let mut is_last_small_char = false;

  for (_index, char) in s.chars().enumerate() {
    if char.is_alphanumeric() {
      if is_last_special_char {
        if result.len() > 0 {
          result.push('-');
        }

        is_last_special_char = false;
      } else if is_last_small_char && char.is_uppercase() && result.len() > 0 {
        result.push('-');
      }

      result.push(char.to_ascii_lowercase());

      if char.is_lowercase() {
        is_last_small_char = true;
      } else {
        is_last_small_char = false;
      }
    } else {
      is_last_special_char = true;
    }
  }

  return result;
}