use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApprovalError {
    NotApproved,
}

impl std::fmt::Display for ApprovalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalError::NotApproved => write!(f, "Export blocked: animation has not been approved by a human operator"),
        }
    }
}

impl std::error::Error for ApprovalError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Approval {
    pub approved: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Default for Approval {
    fn default() -> Self {
        Self {
            approved: false,
            note: None,
        }
    }
}

impl Approval {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn approve(&mut self, note: Option<String>) {
        self.approved = true;
        self.note = note;
    }

    pub fn revoke(&mut self) {
        self.approved = false;
    }

    pub fn is_approved(&self) -> bool {
        self.approved
    }

    pub fn require_approved(&self) -> Result<(), ApprovalError> {
        if self.approved {
            Ok(())
        } else {
            Err(ApprovalError::NotApproved)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_unapproved() {
        let a = Approval::default();
        assert!(!a.is_approved());
        assert!(a.require_approved().is_err());
    }

    #[test]
    fn approve_enables_export() {
        let mut a = Approval::new();
        a.approve(Some("Human review OK".into()));
        assert!(a.is_approved());
        assert!(a.require_approved().is_ok());
        assert_eq!(a.note, Some("Human review OK".into()));
    }

    #[test]
    fn revoke_blocks_export() {
        let mut a = Approval::new();
        a.approve(None);
        a.revoke();
        assert!(!a.is_approved());
        assert!(a.require_approved().is_err());
    }

    #[test]
    fn serde_roundtrip_approved() {
        let a = Approval {
            approved: true,
            note: Some("Director sign-off".into()),
        };
        let json = serde_json::to_string(&a).unwrap();
        let b: Approval = serde_json::from_str(&json).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn serde_roundtrip_unapproved() {
        let a = Approval::default();
        let json = serde_json::to_string(&a).unwrap();
        let b: Approval = serde_json::from_str(&json).unwrap();
        assert_eq!(a, b);
    }
}