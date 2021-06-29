#![warn(missing_docs)]

use sp_std::marker::PhantomData;

use common::origin::ActorOriginValidator;
use proposals_engine::VotersParameters;

use super::{MemberId, MembershipOriginValidator};

/// Handles work with the council.
/// Provides implementations for ActorOriginValidator and VotersParameters.
pub struct CouncilManager<T> {
    marker: PhantomData<T>,
}

impl<T: governance::council::Config + membership::Config>
    ActorOriginValidator<
        <T as frame_system::Config>::Origin,
        MemberId<T>,
        <T as frame_system::Config>::AccountId,
    > for CouncilManager<T>
{
    /// Check for valid combination of origin and actor_id. Actor_id should be valid member_id of
    /// the membership module
    fn ensure_actor_origin(
        origin: <T as frame_system::Config>::Origin,
        actor_id: MemberId<T>,
    ) -> Result<<T as frame_system::Config>::AccountId, &'static str> {
        let account_id = <MembershipOriginValidator<T>>::ensure_actor_origin(origin, actor_id)?;

        if <governance::council::Module<T>>::is_councilor(&account_id) {
            return Ok(account_id);
        }

        Err("Council validation failed: account id doesn't belong to a council member")
    }
}

impl<T: governance::council::Config> VotersParameters for CouncilManager<T> {
    /// Implement total_voters_count() as council size
    fn total_voters_count() -> u32 {
        <governance::council::Module<T>>::active_council().len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::CouncilManager;
    use crate::Runtime;
    use common::origin::ActorOriginValidator;
    use frame_system::RawOrigin;
    use proposals_engine::VotersParameters;
    use sp_runtime::AccountId32;

    type Council = governance::council::Module<Runtime>;

    fn initial_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        t.into()
    }

    type Membership = membership::Module<Runtime>;

    #[test]
    fn council_origin_validator_fails_with_unregistered_member() {
        initial_test_ext().execute_with(|| {
            let origin = RawOrigin::Signed(AccountId32::default());
            let member_id = 1;
            let error = "Membership validation failed: cannot find a profile for a member";

            let validation_result =
                CouncilManager::<Runtime>::ensure_actor_origin(origin.into(), member_id);

            assert_eq!(validation_result, Err(error));
        });
    }

    #[test]
    fn council_origin_validator_succeeds() {
        initial_test_ext().execute_with(|| {
            let councilor1 = AccountId32::default();
            let councilor2: [u8; 32] = [2; 32];
            let councilor3: [u8; 32] = [3; 32];

            assert!(Council::set_council(
                frame_system::RawOrigin::Root.into(),
                vec![councilor1, councilor2.into(), councilor3.into()]
            )
            .is_ok());

            let account_id = AccountId32::default();
            let origin = RawOrigin::Signed(account_id.clone());
            let authority_account_id = AccountId32::default();
            Membership::set_screening_authority(
                RawOrigin::Root.into(),
                authority_account_id.clone(),
            )
            .unwrap();

            Membership::add_screened_member(
                RawOrigin::Signed(authority_account_id).into(),
                account_id.clone(),
                Some(b"handle".to_vec()),
                None,
                None,
                None,
            )
            .unwrap();
            let member_id = 0; // newly created member_id

            let validation_result =
                CouncilManager::<Runtime>::ensure_actor_origin(origin.into(), member_id);

            assert_eq!(validation_result, Ok(account_id));
        });
    }

    #[test]
    fn council_origin_validator_fails_with_incompatible_account_id_and_member_id() {
        initial_test_ext().execute_with(|| {
            let account_id = AccountId32::default();
            let error =
                "Membership validation failed: given account doesn't match with profile accounts";
            let authority_account_id = AccountId32::default();
            Membership::set_screening_authority(
                RawOrigin::Root.into(),
                authority_account_id.clone(),
            )
            .unwrap();

            Membership::add_screened_member(
                RawOrigin::Signed(authority_account_id).into(),
                account_id.clone(),
                Some(b"handle".to_vec()),
                None,
                None,
                None,
            )
            .unwrap();
            let member_id = 0; // newly created member_id

            let invalid_account_id: [u8; 32] = [2; 32];
            let validation_result = CouncilManager::<Runtime>::ensure_actor_origin(
                RawOrigin::Signed(invalid_account_id.into()).into(),
                member_id,
            );

            assert_eq!(validation_result, Err(error));
        });
    }

    #[test]
    fn council_origin_validator_fails_with_not_council_account_id() {
        initial_test_ext().execute_with(|| {
            let account_id = AccountId32::default();
            let origin = RawOrigin::Signed(account_id.clone());
            let error = "Council validation failed: account id doesn't belong to a council member";
            let authority_account_id = AccountId32::default();
            Membership::set_screening_authority(
                RawOrigin::Root.into(),
                authority_account_id.clone(),
            )
            .unwrap();

            Membership::add_screened_member(
                RawOrigin::Signed(authority_account_id).into(),
                account_id,
                Some(b"handle".to_vec()),
                None,
                None,
                None,
            )
            .unwrap();
            let member_id = 0; // newly created member_id

            let validation_result =
                CouncilManager::<Runtime>::ensure_actor_origin(origin.into(), member_id);

            assert_eq!(validation_result, Err(error));
        });
    }

    #[test]
    fn council_size_calculation_aka_total_voters_count_succeeds() {
        initial_test_ext().execute_with(|| {
            let councilor1 = AccountId32::default();
            let councilor2: [u8; 32] = [2; 32];
            let councilor3: [u8; 32] = [3; 32];
            let councilor4: [u8; 32] = [4; 32];
            assert!(Council::set_council(
                frame_system::RawOrigin::Root.into(),
                vec![
                    councilor1,
                    councilor2.into(),
                    councilor3.into(),
                    councilor4.into()
                ]
            )
            .is_ok());

            assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 4)
        });
    }
}
