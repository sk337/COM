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
    /// let string_constant = StringConstant::new("Hello, World!", 0x1000, 0x100C);
    /// assert_eq!(string_constant.value, "Hello, World!");
    /// assert_eq!(string_constant.start, 0x1000);
    /// assert_eq!(string_constant.end, 0x100C);
    /// ```
    pub fn new(value: &str, start: Address, end: Address) -> Self {
        assert_eq!(
            end - start,
            value.bytes().len() as Address,
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
    /// let string_constant = StringConstant::new("Hello, World!", 0x1000, 0x100C);
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
    /// let string_constant = StringConstant::new(b"Hello, World!\r\n$", 0x1000, 0x1010);
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
    /// string_constant_list.0.push(StringConstant::new("Hello, World!", 0x1000, 0x100C));
    /// string_constant_list.0.push(StringConstant::new("Goodbye, World!", 0x100D, 0x1019));
    /// 
    /// assert_eq!(string_constant_list.get_string_constant(0x1000).unwrap().value, "Hello, World!");
    /// assert_eq!(string_constant_list.get_string_constant(0x1009).unwrap().value, "Hello, World!");
    /// 
    /// assert_eq!(string_constant_list.get_string_constant(0x100D).unwrap().value, "Goodbye, World!");
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
