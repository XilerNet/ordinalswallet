#[derive(Debug, Clone)]
pub enum DomainLength {
    SingleCharacter,
    VeryShort,
    Short,
    Normal,
}

impl From<&str> for DomainLength {
    /// Returns the domain length of a given domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - A domain string.
    ///
    /// # Panics
    ///
    /// Panics if the domain does not contain the .o suffix.
    /// Panics if the domain is less than 1 character long.
    ///
    /// # Examples
    ///
    /// ```
    /// let domain = "xiler.o";
    /// let domain_length = DomainLength::from(domain);
    ///
    /// assert!(matches!(domain_length, DomainLength::Normal));
    /// ```
    fn from(domain: &str) -> Self {
        let domain = domain.to_lowercase();

        if !domain.ends_with(".o") {
            panic!("Domain must contain the .o suffix");
        } else if domain.len() < 3 {
            panic!("Domain must be at least 1 character long");
        }

        let domain = &domain[..domain.len() - 2];

        match domain.len() {
            1 => Self::SingleCharacter,
            2 => Self::VeryShort,
            3..4 => Self::Short,
            _ => Self::Normal,
        }
    }
}

impl From<String> for DomainLength {
    fn from(domain: String) -> Self {
        domain.as_str().into()
    }
}

impl ToString for DomainLength {
    fn to_string(&self) -> String {
        match self {
            Self::SingleCharacter => "SingleCharacter".to_string(),
            Self::VeryShort => "VeryShort".to_string(),
            Self::Short => "Short".to_string(),
            Self::Normal => "Normal".to_string(),
        }
    }
}

impl serde::Serialize for DomainLength {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    macro_rules! test_domain_length {
        ($domain:ident,$expected:pat) => {
            paste! {
                #[test]
                fn [<$domain _domain_length_from_str>]() {
                    let domain = format!("{}.o", stringify!($domain));
                    let domain_length = DomainLength::from(domain.as_str());

                    assert!(matches!(domain_length, $expected));
                }

                #[test]
                fn [<$domain _domain_length_from_string>]() {
                    let domain = format!("{}.o", stringify!($domain));
                    let domain_length = DomainLength::from(domain);

                    assert!(matches!(domain_length, $expected));
                }
            }
        };
    }

    macro_rules! test_domain_length_panic {
        ($key:ident,$domain:literal) => {
            paste! {
                #[test]
                #[should_panic]
                fn [<$key _domain_length_from_str_panic>]() {
                    let _ = DomainLength::from($domain);
                }

                #[test]
                #[should_panic]
                fn [<$key _domain_length_from_string_panic>]() {
                    let domain = $domain.to_string();
                    let _ = DomainLength::from(domain);
                }
            }
        };
    }

    macro_rules! test_domain_length_to_string {
        ($domain_length:pat,$expected:ident) => {
            paste! {
                #[test]
                fn [<$expected:snake _domain_length_to_string>]() {
                    let domain_length = $domain_length;
                    let domain_length_string = domain_length.to_string();
                    let expected = stringify!($expected).to_string();

                    assert_eq!(domain_length_string, expected);
                }

                #[test]
                fn [<$expected:snake _domain_length_serialize>]() {
                    let domain_length_string = serde_json::to_string(&$domain_length).unwrap();
                    let expected = format!("\"{}\"", stringify!($expected));

                    assert_eq!(domain_length_string, expected);
                }
            }
        };
    }

    test_domain_length!(x, DomainLength::SingleCharacter);
    test_domain_length!(xi, DomainLength::VeryShort);
    test_domain_length!(xil, DomainLength::Short);
    test_domain_length!(xile, DomainLength::Normal);
    test_domain_length!(xiler, DomainLength::Normal);

    test_domain_length_panic!(empty, "");
    test_domain_length_panic!(no_suffix, "xiler");
    test_domain_length_panic!(no_prefix, ".o");
    test_domain_length_panic!(only_suffix, "o");
    test_domain_length_panic!(only_dot, ".");

    test_domain_length_to_string!(DomainLength::SingleCharacter, SingleCharacter);
    test_domain_length_to_string!(DomainLength::VeryShort, VeryShort);
    test_domain_length_to_string!(DomainLength::Short, Short);
    test_domain_length_to_string!(DomainLength::Normal, Normal);
}
