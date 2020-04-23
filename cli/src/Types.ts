import BN from 'bn.js';
import { ElectionStage, Seat } from '@joystream/types';
import { Option, Vec } from '@polkadot/types';
import { BlockNumber, Balance } from '@polkadot/types/interfaces';
import { DerivedBalances } from '@polkadot/api-derive/types';
import { KeyringPair } from '@polkadot/keyring/types';
import { MemberId } from '@joystream/types/src/members';

// KeyringPair type extended with mandatory "meta.name"
// It's used for accounts/keys management within CLI.
// If not provided in the account json file, the meta.name value is set to "Unnamed Account"
export type NamedKeyringPair = KeyringPair & {
    meta: {
        name: string
    }
}

// Summary of the account information fetched from the api for "account:current" purposes (currently just balances)
export type AccountSummary = {
    balances: DerivedBalances
}

// Object/Tuple containing council/councilElection information (council:info).
// The tuple is useful, because that's how api.queryMulti returns the results.
export type CouncilInfoTuple = Parameters<typeof createCouncilInfoObj>;
export type CouncilInfoObj = ReturnType<typeof createCouncilInfoObj>;
// This function allows us to easily transform the tuple into the object
// and simplifies the creation of consitent Object and Tuple types (seen above).
export function createCouncilInfoObj(
    activeCouncil: Seat[],
    termEndsAt: BlockNumber,
    autoStart: Boolean,
    newTermDuration: BN,
    candidacyLimit: BN,
    councilSize: BN,
    minCouncilStake: Balance,
    minVotingStake: Balance,
    announcingPeriod: BlockNumber,
    votingPeriod: BlockNumber,
    revealingPeriod: BlockNumber,
    round: BN,
    stage: Option<ElectionStage>
) {
    return {
        activeCouncil,
        termEndsAt,
        autoStart,
        newTermDuration,
        candidacyLimit,
        councilSize,
        minCouncilStake,
        minVotingStake,
        announcingPeriod,
        votingPeriod,
        revealingPeriod,
        round,
        stage
    };
}

// Account memberships information retrieved from the api:
// MembershipIds by root account and controller account (for given address/Account)
export type AccountMemberships = {
    rootIds: Vec<MemberId>,
    controllerIds: Vec<MemberId>
};

// Object with "name" and "value" properties, used for rendering simple CLI tables like:
// Total balance:   100 JOY
// Free calance:     50 JOY
export type NameValueObj = { name: string, value: string };
