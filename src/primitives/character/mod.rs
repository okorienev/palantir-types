mod bitmap;
mod impls;

#[derive(Debug)]
pub struct Character(pub u8);

#[cfg(test)]
mod tests {
    use super::bitmap::ALLOWED_UTF8_CHARACTERS;
    use super::Character;
    use deku::prelude::*;

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct CharacterTest(Character);

    #[test]
    fn test_valid_characters() {
        for char_code in ALLOWED_UTF8_CHARACTERS.iter() {
            let data = [*char_code];
            let (_rest, mut val) = CharacterTest::from_bytes((&data, 0)).unwrap();
            assert_eq!(CharacterTest(Character(*char_code)), val);

            let data_out = val.to_bytes().unwrap();
            assert_eq!(data_out, [*char_code]);
        }
    }

    #[test]
    fn test_name_with_allowed_separators() {
        for char_code in String::from("app.controller.action_1-get").as_bytes() {
            let data = [*char_code];
            let (_rest, mut val) = CharacterTest::from_bytes((&data, 0)).unwrap();
            assert_eq!(CharacterTest(Character(*char_code)), val);

            let data_out = val.to_bytes().unwrap();
            assert_eq!(data_out, [*char_code]);
        }
    }

    #[test]
    fn test_invalid_characters() {
        for char_code in String::from(" |\\/*(){}[]$;").as_bytes() {
            let data = [*char_code];
            let result = CharacterTest::from_bytes((&data, 0));

            match result {
                Ok(_0) => panic!("how did u parse invalid chars?"),
                Err(err) => match err {
                    DekuError::InvalidParam(..) => (),
                    _ => panic!("wrong error type"),
                },
            }
        }
    }

    #[test]
    fn test_cyrillic_character() {
        for char_code in String::from("РядокУкраїнськоюУКамелКейсі").as_bytes()
        {
            let data = [*char_code];
            let result = CharacterTest::from_bytes((&data, 0));

            match result {
                Ok(_0) => panic!("how did u parse invalid chars?"),
                Err(err) => match err {
                    DekuError::InvalidParam(..) => (),
                    _ => panic!("wrong error type"),
                },
            }
        }
    }
}
