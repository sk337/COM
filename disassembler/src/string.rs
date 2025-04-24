use crate::consts::Address;

/// A struct representing a string constant in the disassembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringConstant {
    /// the raw value of the string
    pub value: String,
    /// the address of the string
    pub start: Address,
    /// the address of the end of the string
    pub end: Address,
}

impl StringConstant {
    /// Creates a new StringConstant
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the string
    /// * `start` - The address of the start of the string
    /// * `end` - The address of the end of the string
    ///
    /// # Returns
    ///
    /// A new instance of `StringConstant`
    ///
    /// # Examples
    ///
    /// ```
    /// use disassembler::string::StringConstant;
    /// use disassembler::consts::Address;
    ///
    /// let string_constant = StringConstant::new("Hello, World!", 0x1000, 0x100D);
    /// assert_eq!(string_constant.value, "Hello, World!");
    /// assert_eq!(string_constant.start, 0x1000);
    /// assert_eq!(string_constant.end, 0x100D);
    /// ```
    pub fn new(value: &str, start: Address, end: Address) -> Self {
        assert_eq!(
            end - start,
            value.bytes().len().into(),
            "The length of the string does not match the length of the address range"
        );

        StringConstant {
            value: value.to_string(),
            start,
            end,
        }
    }
    /// Returns the length of the string
    ///
    /// # Returns
    ///
    /// The length of the string
    ///
    /// # Examples
    ///
    /// ```
    /// use disassembler::string::StringConstant;
    /// use disassembler::consts::Address;
    ///
    /// let string_constant = StringConstant::new("Hello, World!", 0x1000, 0x100D);
    /// assert_eq!(string_constant.len(), 13);
    /// ```
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns the string constant as a assembly `db` statement
    ///
    /// # Returns
    ///
    /// A string representing the `db` statement
    ///
    /// # Examples
    ///
    /// ```
    /// use disassembler::string::StringConstant;
    /// use disassembler::consts::Address;
    ///
    /// let string_constant = StringConstant::new("Hello, World!\r\n$", 0x1000, 0x1010);
    ///
    /// assert_eq!(string_constant.as_db_statement(), "db \"Hello, World!\", 0x0D, 0x0A, \"$\"");
    /// ```
    pub fn as_db_statement(&self) -> String {
        let mut db_statement = String::from("db ");
        let mut in_quotes = false;

        for byte in self.value.bytes() {
            let is_printable = byte.is_ascii_graphic() || byte == b' ';

            if is_printable {
                if !in_quotes {
                    if !db_statement.ends_with("db ") {
                        db_statement.push_str(", ");
                    }
                    db_statement.push('"');
                    in_quotes = true;
                }
                if byte == b'"' {
                    db_statement.push_str("\\\"");
                } else {
                    db_statement.push(byte as char);
                }
            } else {
                if in_quotes {
                    db_statement.push('"');
                    in_quotes = false;
                }
                if !db_statement.ends_with("db ") && !db_statement.ends_with(", ") {
                    db_statement.push_str(", ");
                }
                db_statement.push_str(&format!("0x{:02X}", byte));
            }
        }

        if in_quotes {
            db_statement.push('"');
        }

        db_statement
    }
}

/// A struct representing a list of string constants
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringConstantList(pub Vec<StringConstant>);

impl StringConstantList {
    /// Creates a new StringConstantList
    ///
    /// # Returns
    ///
    /// A new instance of `StringConstantList` with an empty vector of string constants
    ///
    /// # Examples
    ///
    /// ```
    /// use disassembler::string::StringConstantList;
    ///
    /// let string_constant_list = StringConstantList::new();
    /// assert_eq!(string_constant_list.0.len(), 0);
    /// ```
    pub fn new() -> Self {
        StringConstantList(Vec::new())
    }

    /// return the string that contains the address
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the string constant to search for
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the string constant if found, or `None` if not found
    ///     
    /// # Examples
    ///
    /// ```
    /// use disassembler::string::{StringConstantList, StringConstant};
    /// use disassembler::consts::Address;
    ///
    /// let mut string_constant_list = StringConstantList::new();
    /// string_constant_list.0.push(StringConstant::new("Hello, World!", 0x1000, 0x100D));
    /// string_constant_list.0.push(StringConstant::new("Goodbye, World!", 0x100E, 0x101D));
    ///
    /// assert_eq!(string_constant_list.get_string_constant(0x1000).unwrap().value, "Hello, World!");
    /// assert_eq!(string_constant_list.get_string_constant(0x1009).unwrap().value, "Hello, World!");
    ///
    /// assert_eq!(string_constant_list.get_string_constant(0x100E).unwrap().value, "Goodbye, World!");
    /// assert_eq!(string_constant_list.get_string_constant(0x1013).unwrap().value, "Goodbye, World!");
    ///
    /// assert!(string_constant_list.get_string_constant(0x1020).is_none());
    /// ```
    pub fn get_string_constant(&self, address: Address) -> Option<&StringConstant> {
        self.0
            .iter()
            .find(|s| s.start <= address && s.end >= address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(n: u16) -> Address {
        n
    }

    fn str_const(s: &str, start: u16) -> StringConstant {
        StringConstant::new(s, addr(start), addr(start + s.len() as u16))
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // 1. StringConstant::new
    // ─────────────────────────────────────────────────────────────────────────────
    #[test]
    fn new_string_constant_sets_all_fields() {
        let sc = str_const("abc", 0x2000);
        assert_eq!(sc.value, "abc");
        assert_eq!(sc.start, 0x2000);
        assert_eq!(sc.end, 0x2003);
    }

    #[test]
    #[should_panic(expected = "The length of the string does not match")]
    fn new_panics_if_range_and_length_mismatch() {
        let _ = StringConstant::new("abc", 0x1000, 0x1004); // len = 3 but range = 4
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // 2. StringConstant::len
    // ─────────────────────────────────────────────────────────────────────────────
    #[test]
    fn len_returns_correct_length() {
        assert_eq!(str_const("Hello!", 0x0000).len(), 6);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // 3. StringConstant::as_db_statement
    // ─────────────────────────────────────────────────────────────────────────────
    #[test]
    fn db_statement_printable_only() {
        let s = str_const("abc", 0x0000);
        assert_eq!(s.as_db_statement(), r#"db "abc""#);
    }

    #[test]
    fn db_statement_nonprintables() {
        let s = str_const("\x01\x02\x03", 0x0000);
        assert_eq!(s.as_db_statement(), "db 0x01, 0x02, 0x03");
    }

    #[test]
    fn db_statement_mixed_content() {
        let s = str_const("hi\x0D\x0A$", 0x0000);
        assert_eq!(s.as_db_statement(), r#"db "hi", 0x0D, 0x0A, "$""#);
    }

    #[test]
    fn db_statement_with_space_and_quotes() {
        let s = str_const(r#"A "quote""#, 0x0000);
        assert_eq!(s.as_db_statement(), r#"db "A \"quote\"""#);
    }

    #[test]
    fn db_statement_empty_string() {
        let s = str_const("", 0x0000);
        assert_eq!(s.as_db_statement(), "db ");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // 4. StringConstantList
    // ─────────────────────────────────────────────────────────────────────────────
    #[test]
    fn new_string_constant_list_is_empty() {
        let list = StringConstantList::new();
        assert!(list.0.is_empty());
    }

    #[test]
    fn get_string_constant_returns_containing_string() {
        let mut list = StringConstantList::new();
        list.0.push(str_const("hello", 0x1000)); // 0x1000–0x1005
        list.0.push(str_const("goodbye", 0x1006)); // 0x1006–0x100D

        assert_eq!(list.get_string_constant(0x1000).unwrap().value, "hello");
        assert_eq!(list.get_string_constant(0x1004).unwrap().value, "hello");
        assert_eq!(list.get_string_constant(0x1006).unwrap().value, "goodbye");
        assert_eq!(list.get_string_constant(0x100C).unwrap().value, "goodbye");
    }

    #[test]
    fn get_string_constant_returns_none_if_not_found() {
        let mut list = StringConstantList::new();
        list.0.push(str_const("hi", 0x2000));
        println!("{:?}", list);
        assert!(list.get_string_constant(0x1FFF).is_none());
        assert!(list.get_string_constant(0x2002).is_none()); // just past end
    }

    #[test]
    fn equality_works_for_string_constants_and_lists() {
        let a = str_const("abc", 0x1000);
        let b = str_const("abc", 0x1000);
        let c = str_const("xyz", 0x2000);
        assert_eq!(a, b);
        assert_ne!(a, c);

        let list1 = StringConstantList(vec![a.clone()]);
        let list2 = StringConstantList(vec![b]);
        let list3 = StringConstantList(vec![c]);
        assert_eq!(list1, list2);
        assert_ne!(list1, list3);
    }
}
