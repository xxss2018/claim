#[weight=0]
pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
    let sender = ensure_signed(origin)?;
    ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
    Proofs::<T>::insert(&claim,(sender.clone(), frame_system::Module::<T>block_number()));
    Self::deposit_event(RawEvent::ClaimCreated(sender,claim));
    Ok(())
}

#[weight=0]
pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult{
    let sender = ensure_signed(origin)?;
    ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClainNotExist);
    let (owner, _block_number) = Proofs::<T>::get(&claim);
    ensure!(owner == sender, Error::<T>::NotClaimOwner);
    Proofs::<T>::remove(&claim);
    Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
    Ok(())
}
