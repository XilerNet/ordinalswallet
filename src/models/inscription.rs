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

#[derive(Debug, Clone, Serialize)]
pub struct NewInscriptions {
    pub new_inscriptions: Vec<Inscription>,
    creator_address: String,
    creator_signature: String,
    slug: String,
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

impl NewInscriptions {
    pub fn new(
        new_inscriptions: Vec<Inscription>,
        creator_address: String,
        creator_signature: String,
        slug: String,
    ) -> Self {
        Self {
            new_inscriptions,
            creator_address,
            creator_signature,
            slug,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSCRIPITON_ID: &'static str =
        "bd3bfa98c592fdb6ee81d4655082c43f27b63b05c706bd47bac4e1b715eab7a6i0";

    #[test]
    fn inscription_new() {
        let inscription = Inscription::new(
            INSCRIPITON_ID.to_string(),
            "domain".to_string(),
            vec![InscriptionMetaAttribute::new(
                "trait_type".to_string(),
                "value".to_string(),
            )],
        );

        let meta = inscription
            .meta
            .expect("inscription.meta should not be None");

        let attributes = meta
            .attributes
            .expect("inscription.meta.attributes should not be None");

        assert_eq!(inscription.id, INSCRIPITON_ID);
        assert_eq!(meta.name.unwrap(), "domain");
        assert_eq!(attributes[0].trait_type, "trait_type");
        assert_eq!(attributes[0].value, "value");
    }

    #[test]
    fn inscription_new_no_attributes() {
        let inscription =
            Inscription::new(INSCRIPITON_ID.to_string(), "domain".to_string(), vec![]);

        let meta = inscription
            .meta
            .expect("inscription.meta should not be None");

        assert_eq!(inscription.id, INSCRIPITON_ID);
        assert_eq!(meta.name.unwrap(), "domain");
    }

    #[test]
    fn inscription_new_serialize_json() {
        let inscription = Inscription::new(
            INSCRIPITON_ID.to_string(),
            "domain".to_string(),
            vec![InscriptionMetaAttribute::new(
                "trait_type".to_string(),
                "value".to_string(),
            )],
        );

        let inscription_json = serde_json::to_string(&inscription).unwrap();

        assert_eq!(
            inscription_json,
            r#"{"id":"bd3bfa98c592fdb6ee81d4655082c43f27b63b05c706bd47bac4e1b715eab7a6i0","meta":{"name":"domain","attributes":[{"trait_type":"trait_type","value":"value"}]}}"#
        );
    }

    #[test]
    fn inscription_meta_attribute_new() {
        let attribute =
            InscriptionMetaAttribute::new("trait_type".to_string(), "value".to_string());

        assert_eq!(attribute.trait_type, "trait_type");
        assert_eq!(attribute.value, "value");
    }

    #[test]
    fn inscription_meta_attribute_new_serialize_json() {
        let attribute =
            InscriptionMetaAttribute::new("trait_type".to_string(), "value".to_string());

        let attribute_json = serde_json::to_string(&attribute).unwrap();

        assert_eq!(
            attribute_json,
            r#"{"trait_type":"trait_type","value":"value"}"#
        );
    }

    #[test]
    fn new_inscriptions_new() {
        let new_inscriptions = NewInscriptions::new(
            vec![Inscription::new(
                INSCRIPITON_ID.to_string(),
                "domain".to_string(),
                vec![InscriptionMetaAttribute::new(
                    "trait_type".to_string(),
                    "value".to_string(),
                )],
            )],
            "creator_address".to_string(),
            "creator_signature".to_string(),
            "xiler-dns".to_string(),
        );

        assert_eq!(new_inscriptions.new_inscriptions.len(), 1);
        assert_eq!(new_inscriptions.creator_address, "creator_address");
        assert_eq!(new_inscriptions.creator_signature, "creator_signature");
        assert_eq!(new_inscriptions.slug, "xiler-dns");
    }

    #[test]
    fn new_inscriptions_new_serialize_json() {
        let new_inscriptions = NewInscriptions::new(
            vec![Inscription::new(
                INSCRIPITON_ID.to_string(),
                "domain".to_string(),
                vec![InscriptionMetaAttribute::new(
                    "trait_type".to_string(),
                    "value".to_string(),
                )],
            )],
            "creator_address".to_string(),
            "creator_signature".to_string(),
            "xiler-dns".to_string(),
        );

        let new_inscriptions_json = serde_json::to_string(&new_inscriptions).unwrap();

        assert_eq!(
            new_inscriptions_json,
            r#"{"new_inscriptions":[{"id":"bd3bfa98c592fdb6ee81d4655082c43f27b63b05c706bd47bac4e1b715eab7a6i0","meta":{"name":"domain","attributes":[{"trait_type":"trait_type","value":"value"}]}}],"creator_address":"creator_address","creator_signature":"creator_signature","slug":"xiler-dns"}"#
        );
    }
}
