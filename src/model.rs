use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Inscription {
    id: String,
    meta: Option<InscriptionMeta>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InscriptionMeta {
    pub name: Option<String>,
    pub attributes: Option<Vec<InscriptionMetaAttribute>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InscriptionMetaAttribute {
    pub trait_type: String,
    pub value: String,
}

impl Inscription {
    pub fn new(id: String, domain: String, attributes: Vec<InscriptionMetaAttribute>) -> Self {
        let meta = InscriptionMeta {
            name: Some(domain),
            attributes: Some(attributes),
        };

        Self {
            id,
            meta: Some(meta),
        }
    }
}

impl InscriptionMetaAttribute {
    pub fn new(trait_type: String, value: String) -> Self {
        Self { trait_type, value }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_inscription_new() {
//         let id = "bd3bfa98c592fdb6ee81d4655082c43f27b63b05c706bd47bac4e1b715eab7a6i0".to_string();
//         let domain = "xiler.o".to_string();
//         let attributes = vec![InscriptionMetaAttribute::new(
//             "length".to_string(),
//             "test".to_string(),
//         )];
//
//         let inscription = Inscription::new(id, domain, attributes);
//
//         assert_eq!(inscription.id, "0x1234");
//         assert_eq!(inscription.meta.unwrap().name.unwrap(), "test");
//         assert_eq!(
//             inscription.meta.unwrap().attributes.unwrap()[0].trait_type,
//             "test"
//         );
//         assert_eq!(
//             inscription.meta.unwrap().attributes.unwrap()[0].value,
//             "test"
//         );
//     }
// }
