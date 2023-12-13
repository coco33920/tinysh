pub fn is_an_allowed_character(character: char) -> bool {
    return character.is_alphanumeric()
        || character == '&'
        || character == '>'
        || character == '|'
        || character == '-'
        || character == '<'
        || character == '_'
        || character == '~';
}

#[cfg(test)]
mod test {
    use super::is_an_allowed_character;

    #[test]
    pub fn test_allowed() {
        let expected = vec!['c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~'];
        let value = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '^', '%',
        ];
        let mut final_value = Vec::new();
        value
            .into_iter()
            .filter(|x| is_an_allowed_character(x.clone()))
            .for_each(|f| final_value.push(f));
        assert_eq!(final_value, expected)
    }
}
