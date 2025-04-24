use std::fmt::Display;

use crate::consts::Address;

/// an enum representing the type of comment
/// that can be added to the disassembly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentType {
    /// A comment Before the instruction
    PRE,
    /// A comment After the instruction
    POST,
    /// A comment on the same line as the instruction
    INLINE,
}

/// a struct representing a comment
/// that can be added to the disassembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// the type of comment
    pub comment_type: CommentType,
    /// the comment text
    pub comment_text: String,
    /// the address of the comment
    pub address: Address,
}

impl Comment {
    /// creates a new comment
    /// # Arguments
    /// * `comment_type` - the type of comment
    /// * `comment_text` - the comment text
    pub fn new(comment_type: CommentType, comment_text: String, address: Address) -> Comment {
        Comment {
            comment_type,
            comment_text,
            address,
        }
    }
}

impl Display for Comment {
    /// displays the comment in the format
    /// `; <comment_text>`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "; {}", self.comment_text)
    }
}

/// a struct representing a list of comments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentList(pub Vec<Comment>);

impl CommentList {
    /// creates a new comment list
    ///
    /// # Returns
    ///
    /// A new comment list
    ///
    /// # Example
    ///
    /// ```
    /// use disassembler::comment::CommentList;
    ///
    /// let comment_list = CommentList::new();
    ///
    /// assert_eq!(comment_list.0.len(), 0);
    /// ```
    pub fn new() -> CommentList {
        CommentList(Vec::new())
    }

    /// gets the comments for a given address
    /// # Arguments
    ///
    /// * `address` - the address of the comment
    ///
    /// # Returns
    ///
    /// A vector of references to the comments
    ///
    /// # Example
    ///
    /// ```
    /// use disassembler::comment::{Comment, CommentType, CommentList};
    /// use disassembler::consts::Address;
    ///
    /// let mut comment_list = CommentList::new();
    /// let comment = Comment::new(CommentType::PRE, String::from("This is a comment"), 0x1234);
    /// comment_list.0.push(comment);
    /// let comments = comment_list.get_comments(0x1234);
    /// assert_eq!(comments.len(), 1);
    /// assert_eq!(comments[0].comment_text, "This is a comment");
    /// assert_eq!(comments[0].comment_type, CommentType::PRE);
    ///
    /// ```
    pub fn get_comments(&self, address: Address) -> Vec<&Comment> {
        self.0
            .iter()
            .filter(|comment| comment.address == address)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A tiny helper to avoid repeating boiler-plate
    fn cmt(addr: Address, kind: CommentType, text: &str) -> Comment {
        Comment::new(kind, text.into(), addr)
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 1.  Comment::new populates fields correctly
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn new_comment_has_expected_fields() {
        let c = cmt(0x1000, CommentType::PRE, "Hello");
        assert_eq!(c.address, 0x1000);
        assert_eq!(c.comment_type, CommentType::PRE);
        assert_eq!(c.comment_text, "Hello");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 2.  Display implementation prints “; text”
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn comment_display_prefixes_semicolon() {
        let shown = format!("{}", cmt(0, CommentType::INLINE, "Hi there"));
        assert_eq!(shown, "; Hi there");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 3.  CommentList::new starts empty
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn new_comment_list_is_empty() {
        let list = CommentList::new();
        assert!(list.0.is_empty());
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 4.  get_comments returns all and only matching addresses
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn get_comments_filters_by_address() {
        let mut list = CommentList::new();

        // Two comments at 0x1234, one elsewhere
        let a = cmt(0x1234, CommentType::PRE, "First");
        let b = cmt(0x1234, CommentType::POST, "Second");
        let c = cmt(0x9999, CommentType::INLINE, "Other");

        list.0.extend([a.clone(), b.clone(), c]);

        let hits = list.get_comments(0x1234);
        assert_eq!(hits.len(), 2);
        assert!(hits.contains(&&a));
        assert!(hits.contains(&&b));

        // Unknown address ⇒ empty vec
        assert!(list.get_comments(0xDEAD).is_empty());
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 5.  Equality semantics are structural
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn comment_equality_is_fieldwise() {
        let x = cmt(0x1111, CommentType::INLINE, "Same");
        let y = cmt(0x1111, CommentType::INLINE, "Same");
        let z = cmt(0x1111, CommentType::PRE, "Same but different type");

        assert_eq!(x, y);
        assert_ne!(x, z);
    }
}
