use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for approval violations.
/// Returned when attempting to export without explicit human approval.
#[derive(Error, Debug)]
pub enum ApprovalError {
    #[error("Export blocked: animation has not been approved by a human operator")]
    NotApproved,
}

/// Represents the human approval state.
///
/// This is the single source of truth for export permission.
/// The engine must refuse to render final output if `approved` is `false`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Approval {
    /// Whether the current animation state has been manually reviewed and approved.
    pub approved: bool,

    /// Optional human‑readable note explaining the approval (e.g., reviewer name, date).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Default for Approval {
    /// All new projects start in an *unapproved* state.
    /// Export is impossible until a human explicitly sets `approved = true`.
    fn default() -> Self {
        Self {
            approved: false,
            note: None,
        }
    }
}

impl Approval {
    /// Construct a new, unapproved approval entry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark the animation as approved.  
    /// This must be invoked by a human‑controlled UI action only.
    pub fn approve(&mut self, note: Option<String>) {
        self.approved = true;
        self.note = note;
    }

    /// Revoke a previous approval, blocking further exports.
    pub fn revoke(&mut self) {
        self.approved = false;
        // Optionally keep the note for audit trail; clearing it may also be desired.
    }

    /// Returns `true` if export is permitted.
    pub fn is_approved(&self) -> bool {
        self.approved
    }

    /// Hard‑check for export pipelines.  
    /// Returns `Ok(())` if approved, otherwise `Err(ApprovalError::NotApproved)`.
    /// Export **must** abort on error.
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
            note: Some("Director sign‑off".into()),
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