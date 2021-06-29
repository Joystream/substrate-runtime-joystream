use frame_system::RawOrigin;

/// Abstract validator for the origin(account_id) and actor_id (eg.: thread author id).
pub trait ActorOriginValidator<Origin, ActorId, AccountId> {
    /// Check for valid combination of origin and actor_id.
    fn ensure_actor_origin(origin: Origin, actor_id: ActorId) -> Result<AccountId, &'static str>;
}

// TODO: delete when T::Origin will support the clone()
/// Multiplies the T::Origin.
/// In our current substrate version frame_system::Origin doesn't support clone(),
/// but it will be supported in latest up-to-date substrate version.
pub fn double_origin<T: frame_system::Config>(origin: T::Origin) -> (T::Origin, T::Origin) {
    let coerced_origin = origin.into().ok().unwrap_or(RawOrigin::None);

    let (cloned_origin1, cloned_origin2) = match coerced_origin {
        RawOrigin::None => (RawOrigin::None, RawOrigin::None),
        RawOrigin::Root => (RawOrigin::Root, RawOrigin::Root),
        RawOrigin::Signed(account_id) => (
            RawOrigin::Signed(account_id.clone()),
            RawOrigin::Signed(account_id),
        ),
    };

    (cloned_origin1.into(), cloned_origin2.into())
}
