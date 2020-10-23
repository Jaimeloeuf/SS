// pub enum ScalarTypes
lazy_static! {
    static ref TYPES: HashMap<String, TokenType> = {
        let mut m = HashMap::new();

        m.insert("u8".to_string(), TokenType::u8);
        m.insert("u16".to_string(), TokenType::u16);
        m.insert("u32".to_string(), TokenType::u32);
        m.insert("u64".to_string(), TokenType::u64);
        m.insert("i8".to_string(), TokenType::i8);
        m.insert("i16".to_string(), TokenType::i16);
        m.insert("i32".to_string(), TokenType::i32);
        m.insert("i64".to_string(), TokenType::i64);
        m.insert("l128".to_string(), TokenType::l128);
        m.insert("f32".to_string(), TokenType::f32);
        m.insert("f64".to_string(), TokenType::f64);

        // Aliases
        m.insert("ubyte".to_string(), TokenType::u8);
        m.insert("byte".to_string(), TokenType::i8);
        m.insert("uint".to_string(), TokenType::u32);
        m.insert("int".to_string(), TokenType::i32);
        m.insert("ulong".to_string(), TokenType::u128);
        m.insert("long".to_string(), TokenType::i128);
        m.insert("float".to_string(), TokenType::f32);

        m.insert("string".to_string(), TokenType::string);
        m.insert("bool".to_string(), TokenType::bool);
        m
    };
}
