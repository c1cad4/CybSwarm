//! Deterministic coordination of two agent outputs with independent evaluation.
//! This does not execute untrusted code or invoke language models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub author: String,
    pub claim: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Review {
    Accepted,
    Rejected(&'static str),
}

pub fn independent_review(proposal: &Evidence, verification: &Evidence) -> Review {
    if proposal.author == verification.author {
        return Review::Rejected("self-review is not independent");
    }
    if proposal.source.is_empty() || verification.source.is_empty() {
        return Review::Rejected("missing evidence source");
    }
    if proposal.claim != verification.claim {
        return Review::Rejected("claims disagree");
    }
    Review::Accepted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(author: &str, claim: &str) -> Evidence {
        Evidence {
            author: author.into(),
            claim: claim.into(),
            source: "test-fixture".into(),
        }
    }

    #[test]
    fn independent_agreement_is_accepted() {
        assert_eq!(
            independent_review(&evidence("researcher", "A"), &evidence("evaluator", "A")),
            Review::Accepted
        );
    }

    #[test]
    fn self_review_is_rejected() {
        assert_eq!(
            independent_review(&evidence("agent", "A"), &evidence("agent", "A")),
            Review::Rejected("self-review is not independent")
        );
    }

    #[test]
    fn disagreement_is_rejected() {
        assert_eq!(
            independent_review(&evidence("researcher", "A"), &evidence("evaluator", "B")),
            Review::Rejected("claims disagree")
        );
    }

    #[test]
    fn missing_source_is_rejected() {
        let mut reviewer = evidence("evaluator", "A");
        reviewer.source.clear();
        assert_eq!(
            independent_review(&evidence("researcher", "A"), &reviewer),
            Review::Rejected("missing evidence source")
        );
    }
}
