#[cfg(test)]
mod item {
    use bgm_wiki_parser::{item, Item};

    #[test]
    fn single() {
        let (_, res) = item("[e]").unwrap();
        assert_eq!(res, Item::Single("e".to_string()));
    }

    #[test]
    fn named() {
        let (_, res) = item("[a|b]").unwrap();
        assert_eq!(res, Item::Named("a".to_string(), "b".to_string()));
    }

    #[test]
    fn none() {
        let (_, res) = item("[]").unwrap();
        assert_eq!(res, Item::None);
    }
}

#[cfg(test)]
mod data {
    use bgm_wiki_parser::{data, Data, Item};

    #[test]
    fn scalar() {
        let (_, res) = data("any").unwrap();
        assert_eq!(res, Data::Scalar("any".to_string()));
    }

    #[test]
    fn array() {
        let (_, res) = data("{\n[a]\n[b]\n}").unwrap();
        assert_eq!(
            res,
            Data::Array(vec![
                Item::Single("a".to_string()),
                Item::Single("b".to_string())
            ])
        );
    }
}

#[cfg(test)]
mod wiki {
    use bgm_wiki_parser::{wiki, Wiki};

    mod de {
        use bgm_wiki_parser::{Item, Wiki};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        enum V {
            Null,
            Named { k: String, v: String },
            Single { v: Option<String> },
        }

        impl Into<Item> for V {
            fn into(self) -> Item {
                match self {
                    Self::Single { v: Some(s) } => Item::Single(s),
                    Self::Named { k, v } => Item::Named(k, v),
                    Self::Single { v: None } | Self::Null => Item::None,
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        enum D {
            Array(Vec<V>),
            Scalar(String),
        }

        impl Default for D {
            fn default() -> Self {
                Self::Scalar("".to_string())
            }
        }

        impl Into<bgm_wiki_parser::Data> for D {
            fn into(self) -> bgm_wiki_parser::Data {
                match self {
                    Self::Array(v) => {
                        bgm_wiki_parser::Data::Array(v.into_iter().map(Into::into).collect())
                    }
                    Self::Scalar(s) => bgm_wiki_parser::Data::Scalar(s),
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Data {
            pub key: String,
            pub value: Option<D>,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct De {
            pub r#type: Option<String>,
            pub data: Option<Vec<Data>>,
        }

        impl Into<Wiki> for De {
            fn into(self) -> Wiki {
                Wiki {
                    kind: self.r#type.unwrap_or_default(),
                    data: self
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .map(|Data { key, value }| (key, value.unwrap_or_default().into()))
                        .collect(),
                }
            }
        }

        pub fn deserialize(s: &str) -> Wiki {
            let d: De = serde_yaml::from_str(s).unwrap();
            d.into()
        }
    }

    #[test]
    fn array_contain_close() {
        const DATA: &str = include_str!("valid/array_contain_close.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/array_contain_close.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn array_contain_open() {
        const DATA: &str = include_str!("valid/array_contain_open.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/array_contain_open.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn array_multi_sep() {
        const DATA: &str = include_str!("valid/array_multi_sep.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/array_multi_sep.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn array_with_name() {
        const DATA: &str = include_str!("valid/array_with_name.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/array_with_name.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn array() {
        const DATA: &str = include_str!("valid/array.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/array.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn null() {
        const DATA: &str = include_str!("valid/null.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/null.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn scalar_multi_space() {
        const DATA: &str = include_str!("valid/scalar_multi_space.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/scalar_multi_space.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn scalar_space() {
        const DATA: &str = include_str!("valid/scalar_space.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/scalar_space.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn scalar() {
        const DATA: &str = include_str!("valid/scalar.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/scalar.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn r#type() {
        const DATA: &str = include_str!("valid/type.wiki");
        let (_, res) = wiki(DATA).unwrap();
        let correct = de::deserialize(include_str!("valid/type.yaml"));
        assert_eq!(res, correct);
    }

    #[test]
    fn empty() {
        let (_, res) = wiki("").unwrap();
        assert_eq!(res, Wiki::default());
        let (_, res) = wiki("{{Infobox\n}}").unwrap();
        assert_eq!(res, Wiki::default());
    }
}
