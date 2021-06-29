#![warn(missing_docs)]

use sp_std::marker::PhantomData;

use common::origin::ActorOriginValidator;
use frame_system::ensure_signed;

/// Member of the Joystream organization
pub type MemberId<T> = <T as membership::Config>::MemberId;

/// Default membership actor origin validator.
pub struct MembershipOriginValidator<T> {
    marker: PhantomData<T>,
}

impl<T: membership::Config>
    ActorOriginValidator<
        <T as frame_system::Config>::Origin,
        MemberId<T>,
        <T as frame_system::Config>::AccountId,
    > for MembershipOriginValidator<T>
{
    /// Check for valid combination of origin and actor_id. Actor_id should be valid member_id of
    /// the membership module
    fn ensure_actor_origin(
        origin: <T as frame_system::Config>::Origin,
        actor_id: MemberId<T>,
    ) -> Result<<T as frame_system::Config>::AccountId, &'static str> {
        // check valid signed account_id
        let account_id = ensure_signed(origin)?;

        // check whether actor_id belongs to the registered member
        let profile_result = <membership::Module<T>>::ensure_membership(actor_id);

        if let Ok(profile) = profile_result {
            // whether the account_id belongs to the actor
            if profile.controller_account == account_id {
                return Ok(account_id);
            } else {
                return Err("Membership validation failed: given account doesn't match with profile accounts");
            }
        }

        Err("Membership validation failed: cannot find a profile for a member")
    }
}

#[cfg(test)]
mod tests {
    use super::MembershipOriginValidator;
    use crate::Runtime;
    use common::origin::ActorOriginValidator;
    use frame_system::RawOrigin;
    use sp_runtime::AccountId32;

    type Membership = membership::Module<Runtime>;

    fn initial_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        t.into()
    }

    #[test]
    fn membership_origin_validator_fails_with_unregistered_member() {
        initial_test_ext().execute_with(|| {
            let origin = RawOrigin::Signed(AccountId32::default());
            let member_id = 1;
            let error = "Membership validation failed: cannot find a profile for a member";

            let validation_result =
                MembershipOriginValidator::<Runtime>::ensure_actor_origin(origin.into(), member_id);

            assert_eq!(validation_result, Err(error));
        });
    }

    #[test]
    fn membership_origin_validator_succeeds() {
        initial_test_ext().execute_with(|| {
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
                MembershipOriginValidator::<Runtime>::ensure_actor_origin(origin.into(), member_id);

            assert_eq!(validation_result, Ok(account_id));
        });
    }

    #[test]
    fn membership_origin_validator_fails_with_incompatible_account_id_and_member_id() {
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
                account_id,
                Some(b"handle".to_vec()),
                None,
                None,
                None,
            )
            .unwrap();
            let member_id = 0; // newly created member_id

            let invalid_account_id: [u8; 32] = [2; 32];
            let validation_result = MembershipOriginValidator::<Runtime>::ensure_actor_origin(
                RawOrigin::Signed(invalid_account_id.into()).into(),
                member_id,
            );

            assert_eq!(validation_result, Err(error));
        });
    }
}
