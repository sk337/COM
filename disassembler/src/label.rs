use crate::consts::Address;
use std::fmt::Display;

/// An enum to represent the type of label
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelType {
    /// A basic label detected via Jmp
    LABEL,
    /// A function label detected via Call
    FUNCTION,
    /// A data label detected via being used in syscalls such as 0x09
    DATA,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct to represent a label in the disassembled code
pub struct Label {
    /// The address of the label
    pub address: Address,
    /// The type of label
    pub label_type: LabelType,
    /// The name of the label
    pub name: String,
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.label_type {
            LabelType::LABEL => write!(f, "{}: ; label", self.name),
            LabelType::FUNCTION => write!(f, "{}: ; function", self.name),
            LabelType::DATA => write!(f, "{}: ; data", self.name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A wrapper type around Vec<label> for implementing Display
pub struct LabelList(pub Vec<Label>);

impl LabelList {
    /// Creates a new LabelList
    ///
    /// # Returns
    ///
    /// A new instance of `LabelList` with an empty vector of instructions
    /// 
    /// # Examples
    /// 
    /// ```
    /// use disassembler::label::{LabelList, Label, LabelType};
    /// use disassembler::consts::Address;
    /// 
    /// let mut label_list = LabelList::new();
    /// label_list.0.push(Label {
    ///     address: 0x1234,
    ///     label_type: LabelType::LABEL,
    ///     name: String::from("my_label"),
    /// });
    /// 
    /// assert_eq!(label_list.0.len(), 1);
    /// assert_eq!(label_list.0[0].address, 0x1234);
    /// assert_eq!(label_list.0[0].label_type, LabelType::LABEL);
    /// assert_eq!(label_list.0[0].name, "my_label");
    /// ```
    pub fn new() -> Self {
        LabelList(Vec::new())
    }

    /// get a label by its address
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the label to search for
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the label if found, or `None` if not found
    /// 
    /// # Examples
    /// 
    /// ```
    /// use disassembler::label::{LabelList, Label, LabelType};
    /// use disassembler::consts::Address;
    /// 
    /// let mut label_list = LabelList::new();
    /// label_list.0.push(Label {
    ///     address: 0x1234,
    ///     label_type: LabelType::LABEL,
    ///     name: String::from("my_label"),
    /// });
    /// 
    /// let label = label_list.get_by_address(0x1234);
    /// 
    /// assert!(label.is_some());
    /// assert_eq!(label.unwrap().address, 0x1234);
    /// assert_eq!(label.unwrap().label_type, LabelType::LABEL);
    /// assert_eq!(label.unwrap().name, "my_label");
    /// 
    /// ```
    pub fn get_by_address(&self, address: Address) -> Option<&Label> {
        self.0.iter().find(|label| label.address == address)
    }
}

impl Display for LabelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for label in self.0.iter() {
            write!(f, "{}\n", label)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to cut some boilerplate
    fn lbl(addr: Address, kind: LabelType, name: &str) -> Label {
        Label {
            address: addr,
            label_type: kind,
            name: name.into(),
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 1.  Construction basics
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn new_label_list_is_empty() {
        let list = LabelList::new();
        assert!(list.0.is_empty(), "LabelList::new() must start empty");
        assert_eq!(format!("{list}"), "");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 2.  get_by_address lookup
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn lookup_returns_correct_label() {
        let mut list = LabelList::new();
        let expected = lbl(0x1234, LabelType::FUNCTION, "FUNC");
        list.0.push(expected.clone());

        let found = list.get_by_address(0x1234).expect("label must be found");
        assert_eq!(found, &expected);

        // Non-existent address → None
        assert!(list.get_by_address(0xDEAD).is_none());
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 3.  Display formatting – individual labels
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn label_display_variants() {
        assert_eq!(
            format!("{}", lbl(0, LabelType::LABEL,    "LBL")),
            "LBL: ; label"
        );
        assert_eq!(
            format!("{}", lbl(0, LabelType::FUNCTION, "FUNC")),
            "FUNC: ; function"
        );
        assert_eq!(
            format!("{}", lbl(0, LabelType::DATA,     "DATA")),
            "DATA: ; data"
        );
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 4.  Display formatting – whole list
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn label_list_display_lists_each_on_its_own_line() {
        let list = LabelList(vec![
            lbl(0x100, LabelType::LABEL,    "LBL1"),
            lbl(0x120, LabelType::FUNCTION, "FUNC2"),
        ]);

        let expected = "LBL1: ; label\nFUNC2: ; function\n";
        assert_eq!(format!("{list}"), expected);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 5.  Equality semantics
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn label_equality_is_structural() {
        let a = lbl(0x100, LabelType::DATA, "DATA_100");
        let b = lbl(0x100, LabelType::DATA, "DATA_100");
        let c = lbl(0x101, LabelType::DATA, "DATA_101");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
