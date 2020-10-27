// pub enum ScalarTypes
lazy_static! {
    static ref TYPES: HashMap<String, TokenType> = {
        let mut m = HashMap::new();

        m.insert("u8".to_string(), TokenType::type_u8);
        m.insert("u16".to_string(), TokenType::type_u16);
        m.insert("u32".to_string(), TokenType::type_u32);
        m.insert("u64".to_string(), TokenType::type_u64);
        m.insert("i8".to_string(), TokenType::type_i8);
        m.insert("i16".to_string(), TokenType::type_i16);
        m.insert("i32".to_string(), TokenType::type_i32);
        m.insert("i64".to_string(), TokenType::type_i64);
        m.insert("l128".to_string(), TokenType::type_l128);
        m.insert("f32".to_string(), TokenType::type_f32);
        m.insert("f64".to_string(), TokenType::type_f64);

        // Aliases
        m.insert("ubyte".to_string(), TokenType::type_u8);
        m.insert("byte".to_string(), TokenType::type_i8);
        m.insert("uint".to_string(), TokenType::type_u32);
        m.insert("int".to_string(), TokenType::type_i32);
        m.insert("ulong".to_string(), TokenType::type_u128);
        m.insert("long".to_string(), TokenType::type_i128);
        m.insert("float".to_string(), TokenType::type_f32);

        // HMMMMMMM, should this be like rust?
        // Or diff? Since anyways, everything is constant reference,
        // so although things like user string input needs to be stored on the
        // heap, the ref to it wont ever change.
        // Hmm, to make things simple, we should just keep all strings to
        m.insert("string".to_string(), TokenType::type_string);
        m.insert("bool".to_string(), TokenType::type_bool);
        m
    };
}
