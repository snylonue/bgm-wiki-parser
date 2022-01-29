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
mod wiki {}
