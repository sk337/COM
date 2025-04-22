use std::fmt::Display;
use crate::consts::Address;

/// An enum to represent the type of label
#[derive(Debug)]
pub enum LabelType {
    /// A basic label detected via Jmp
    LABEL,
    /// A function label detected via Call
    FUNCTION,
    /// A data label detected via being used in syscalls such as 0x09
    DATA,
}

#[derive(Debug)]
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

#[derive(Debug)]
/// A wrapper type around Vec<label> for implementing Display
pub struct LabelList(pub Vec<Label>);

impl LabelList {
    /// Creates a new LabelList
    /// 
    /// # Returns
    /// 
    /// A new instance of `LabelList` with an empty vector of instructions
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
