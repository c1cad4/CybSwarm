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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub agent_id: String,
    pub claim: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusError {
    InsufficientReviewers,
    DuplicateAgent,
    MissingSource,
    Disagreement,
}

/// Require at least two distinct identities and unanimous matching claims.
/// Agent IDs are unverified strings; this is not Sybil resistance or truth validation.
pub fn unanimous_review(votes: &[Vote]) -> Result<String, ConsensusError> {
    if votes.len() < 2 {
        return Err(ConsensusError::InsufficientReviewers);
    }
    let mut seen = std::collections::HashSet::new();
    for vote in votes {
        if !seen.insert(vote.agent_id.as_str()) {
            return Err(ConsensusError::DuplicateAgent);
        }
        if vote.source.trim().is_empty() {
            return Err(ConsensusError::MissingSource);
        }
        if vote.claim != votes[0].claim {
            return Err(ConsensusError::Disagreement);
        }
    }
    Ok(votes[0].claim.clone())
}

#[cfg(test)]
mod consensus_tests {
    use super::*;

    fn vote(agent_id: &str, claim: &str) -> Vote {
        Vote {
            agent_id: agent_id.into(),
            claim: claim.into(),
            source: "test-source".into(),
        }
    }

    #[test]
    fn two_distinct_agents_agree() {
        assert_eq!(
            unanimous_review(&[vote("a", "result"), vote("b", "result")]),
            Ok("result".into())
        );
    }

    #[test]
    fn duplicate_identity_rejected() {
        assert_eq!(
            unanimous_review(&[vote("a", "result"), vote("a", "result")]),
            Err(ConsensusError::DuplicateAgent)
        );
    }

    #[test]
    fn disagreement_rejected() {
        assert_eq!(
            unanimous_review(&[vote("a", "result"), vote("b", "other")]),
            Err(ConsensusError::Disagreement)
        );
    }

    #[test]
    fn single_agent_rejected() {
        assert_eq!(
            unanimous_review(&[vote("a", "result")]),
            Err(ConsensusError::InsufficientReviewers)
        );
    }
}
