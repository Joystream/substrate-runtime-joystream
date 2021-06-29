use joystream_node_runtime::ProposalsConfigParameters;

/// Development chain config. 0 grace period for all proposals, ie.
/// proposals executed immediatly. Short voting period.
pub fn development() -> ProposalsConfigParameters {
    ProposalsConfigParameters::with_grace_and_voting_periods(0, 200)
}

/// Staging chain config. Shorter grace periods and voting periods than default.
pub fn staging() -> ProposalsConfigParameters {
    ProposalsConfigParameters::with_grace_and_voting_periods(20, 30)
}

/// The default configuration as defined in the runtime module
pub fn production() -> ProposalsConfigParameters {
    ProposalsConfigParameters::default()
}
