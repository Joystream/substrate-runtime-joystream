#![cfg(test)]

use super::mock::*;
use crate::*;
use sp_runtime::traits::Hash;

type TestHash = <Test as frame_system::Trait>::Hash;
type TestHashing = <Test as frame_system::Trait>::Hashing;
type LemmaItemTest = LemmaItem<TestHash>;
type TestProof<Value> = Proof<TestHashing, Value>;

#[derive(Debug)]
struct IndexItem {
    index: usize,
    side: Side,
}

fn helper_index_path(len: usize, index: usize) -> Vec<IndexItem> {
    // used as a helper function to generate the correct sequence of indexes used to
    // construct the merkle path necessary for membership proof
    let mut idx = index;
    assert!(idx > 0); // index starting at 1
    let floor_2 = |x: usize| (x >> 1) + (x % 2);
    let mut path = Vec::new();
    let mut prev_len: usize = 0;
    let mut el = len;
    while el != 1 {
        if idx % 2 == 1 && idx == el {
            path.push(IndexItem {
                index: prev_len + idx,
                side: Side::Left,
            });
        } else {
            match idx % 2 {
                1 => path.push(IndexItem {
                    index: prev_len + idx + 1,
                    side: Side::Right,
                }),
                _ => path.push(IndexItem {
                    index: prev_len + idx - 1,
                    side: Side::Left,
                }),
            };
        }
        prev_len += el;
        idx = floor_2(idx);
        el = floor_2(el);
    }
    return path;
}
fn generate_merkle_root<E: Encode>(collection: &[E]) -> Result<Vec<TestHash>, &'static str> {
    // generates merkle root from the ordered sequence collection.
    // The resulting vector is structured as follows: elements in range
    // [0..collection.len()) will be the tree leaves (layer 0), elements in range
    // [collection.len()..collection.len()/2) will be the nodes in the next to last layer (layer 1)
    // [layer_n_length..layer_n_length/2) will be the number of nodes in layer(n+1)
    if collection.len() == 0 {
        return Err("empty vector");
    }
    let mut out = Vec::new();
    for e in collection.iter() {
        out.push(TestHashing::hash(&e.encode()));
    }

    let mut start: usize = 0;
    let mut last_len = out.len();
    //let mut new_len = out.len();
    let mut max_len = last_len >> 1;
    let mut rem = last_len % 2;

    // range [last..(maxlen >> 1) + (maxlen % 2)]
    while max_len != 0 {
        last_len = out.len();
        for i in 0..max_len {
            out.push(TestHashing::hash(
                &[out[start + 2 * i], out[start + 2 * i + 1]].encode(),
            ));
        }
        if rem == 1 {
            out.push(TestHashing::hash(
                &[out[last_len - 1], out[last_len - 1]].encode(),
            ));
        }
        let new_len: usize = out
            .len()
            .checked_sub(last_len)
            .ok_or("unsigned underflow")?;
        rem = new_len % 2;
        max_len = new_len >> 1;
        start = last_len;
    }
    Ok(out)
}

fn helper_build_merkle_path<E: Encode + Clone>(
    collection: &[E],
    idx: usize,
    merkle_tree: &[TestHash],
) -> Vec<LemmaItemTest> {
    // builds the actual merkle path with the hashes needed for the proof
    let index_path = helper_index_path(collection.len(), idx + 1);
    // for el in index_path.iter() {
    //     println!("{:?}", el)
    // }
    index_path
        .iter()
        .map(|idx_item| LemmaItemTest {
            hash: merkle_tree[idx_item.index - 1],
            side: idx_item.side,
        })
        .collect()
}

#[test]
fn update_maximum_reward_allowed() {
    with_default_mock_builder(|| {
        run_to_block(1);
        let new_amount = minting::BalanceOf::<Test>::from(2_000u32);
        let _res = Content::update_max_reward_allowed(Origin::root(), new_amount);
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::MaxRewardUpdated(new_amount))
        );
    })
}

#[test]
fn update_minimum_cashout_allowed() {
    with_default_mock_builder(|| {
        run_to_block(1);
        let new_amount = minting::BalanceOf::<Test>::from(10u32);
        let _res = Content::update_min_cashout_allowed(Origin::root(), new_amount);
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::MinCashoutUpdated(new_amount))
        );
    })
}

#[test]
fn update_commitment_value() {
    with_default_mock_builder(|| {
        let mut commit = TestHashing::hash(&1.encode());
        let mut _res = Content::update_commitment(Origin::root(), commit);
        run_to_block(1);
        commit = TestHashing::hash(&2.encode());
        _res = Content::update_commitment(Origin::root(), commit);
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::CommitmentUpdated(commit))
        );
    })
}

#[test]
fn update_commitment_with_same_value() {
    with_default_mock_builder(|| {
        let mut commit = TestHashing::hash(&1.encode());
        let mut _res = Content::update_commitment(Origin::root(), commit);
        run_to_block(1);
        commit = TestHashing::hash(&1.encode());
        _res = Content::update_commitment(Origin::root(), commit);
        assert_eq!(System::events().last(), None);
    })
}

#[test]
fn channel_reward_update_test() {
    with_default_mock_builder(|| {
        run_to_block(1); // in order to produce events

        // create payment elements collection
        let ids: [u64; 5] = [1, 2, 3, 4, 5];

        // create channels
        let _ = Content::create_channel(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: Some(FIRST_MEMBER_ORIGIN),
            },
        );

        let _ = Content::create_channel(
            Origin::signed(SECOND_MEMBER_ORIGIN),
            ContentActor::Member(SECOND_MEMBER_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: Some(SECOND_MEMBER_ORIGIN),
            },
        );

        let pull_payments_collection: Vec<PullPaymentElement<Test>> = ids
            .iter()
            .map(|&i| PullPaymentElement::<Test> {
                channel_id: ChannelId::from(FIRST_MEMBER_ID),
                amount_due: minting::BalanceOf::<Test>::from(i),
                reason: TestHashing::hash(&i.encode()),
            })
            .collect::<Vec<PullPaymentElement<Test>>>();

        // generate hash tree and get its root
        let out = generate_merkle_root(&pull_payments_collection).unwrap();
        let merkle_root = out.last().copied().unwrap();

        // set the commitment
        let mut _res = Content::update_commitment(Origin::root(), merkle_root);

        // suppose now channel 1 is trying to collect its payment
        let test_id = 2u64;
        let reward_element = PullPaymentElement::<Test> {
            channel_id: ChannelId::from(FIRST_MEMBER_ID),
            amount_due: minting::BalanceOf::<Test>::from(test_id),
            reason: TestHashing::hash(&test_id.encode()),
        };

        let proof_path =
            helper_build_merkle_path(&pull_payments_collection, (test_id as usize) - 1, &out);

        let proof = TestProof {
            data: reward_element,
            path: proof_path,
        };

        // attempt should succeed
        _res = Content::update_channel_reward(Origin::signed(FIRST_MEMBER_ORIGIN), proof);
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelRewardUpdated(
                reward_element.amount_due,
                reward_element.channel_id,
            ))
        );
    })
}
