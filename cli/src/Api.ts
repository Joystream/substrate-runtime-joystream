import BN from 'bn.js';
import { registerJoystreamTypes } from '@joystream/types';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { QueryableStorageMultiArg } from '@polkadot/api/types';
import { formatBalance } from '@polkadot/util';
import { Hash } from '@polkadot/types/interfaces';
import { KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';
import { AccountSummary, CouncilInfoObj, CouncilInfoTuple, createCouncilInfoObj, AccountMemberships } from './Types';
import { DerivedFees, DerivedBalances } from '@polkadot/api-derive/types';
import { CLIError } from '@oclif/errors';
import { Option, Vec } from '@polkadot/types';
import { MemberId, PaidTermId, PaidMembershipTerms, Profile } from '@joystream/types/src/members';

import ExitCodes from './ExitCodes';

export const DEFAULT_API_URI = 'wss://rome-rpc-endpoint.joystream.org:9944/';
export const TOKEN_SYMBOL = 'JOY';

// Api wrapper for handling most common api calls and allowing easy API implementation switch in the future

export default class Api {
    private _api: ApiPromise;

    private constructor(originalApi:ApiPromise) {
        this._api = originalApi;
    }

    public getOriginalApi(): ApiPromise {
        return this._api;
    }

    private static async initApi(apiUri: string = DEFAULT_API_URI): Promise<ApiPromise> {
        formatBalance.setDefaults({ unit: TOKEN_SYMBOL });
        const wsProvider:WsProvider = new WsProvider(apiUri);
        registerJoystreamTypes();

        return await ApiPromise.create({ provider: wsProvider });
    }

    static async create(apiUri: string = DEFAULT_API_URI): Promise<Api> {
        const originalApi: ApiPromise = await Api.initApi(apiUri);
        return new Api(originalApi);
    }

    private async queryMultiOnce(queries: Parameters<typeof ApiPromise.prototype.queryMulti>[0]): Promise<Codec[]> {
        let results: Codec[] = [];

        const unsub = await this._api.queryMulti(
            queries,
            (res) => { results = res }
        );
        unsub();

        if (!results.length || results.length !== queries.length) {
            throw new CLIError('API querying issue', { exit: ExitCodes.ApiError });
        }

        return results;
    }

    async getAccountsBalancesInfo(accountAddresses:string[]): Promise<DerivedBalances[]> {
        let accountsBalances: DerivedBalances[] = await this._api.derive.balances.votingBalances(accountAddresses);

        return accountsBalances;
    }

    // Get on-chain data related to given account.
    // For now it's just account balances
    async getAccountSummary(accountAddresses:string): Promise<AccountSummary> {
        const balances: DerivedBalances = (await this.getAccountsBalancesInfo([accountAddresses]))[0];
        // TODO: Some more information can be fetched here in the future

        return { balances };
    }

    async getCouncilInfo(): Promise<CouncilInfoObj> {
        const queries: { [P in keyof CouncilInfoObj]: QueryableStorageMultiArg<"promise"> } = {
            activeCouncil:    this._api.query.council.activeCouncil,
            termEndsAt:       this._api.query.council.termEndsAt,
            autoStart:        this._api.query.councilElection.autoStart,
            newTermDuration:  this._api.query.councilElection.newTermDuration,
            candidacyLimit:   this._api.query.councilElection.candidacyLimit,
            councilSize:      this._api.query.councilElection.councilSize,
            minCouncilStake:  this._api.query.councilElection.minCouncilStake,
            minVotingStake:   this._api.query.councilElection.minVotingStake,
            announcingPeriod: this._api.query.councilElection.announcingPeriod,
            votingPeriod:     this._api.query.councilElection.votingPeriod,
            revealingPeriod:  this._api.query.councilElection.revealingPeriod,
            round:            this._api.query.councilElection.round,
            stage:            this._api.query.councilElection.stage
        }
        const results: CouncilInfoTuple = <CouncilInfoTuple> await this.queryMultiOnce(Object.values(queries));

        return createCouncilInfoObj(...results);
    }

    // TODO: This formula is probably not too good, so some better implementation will be required in the future
    async estimateFee(account: KeyringPair, recipientAddr: string, amount: BN): Promise<BN> {
        const transfer = this._api.tx.balances.transfer(recipientAddr, amount);
        const signature = account.sign(transfer.toU8a());
        const transactionByteSize:BN = new BN(transfer.encodedLength + signature.length);

        const fees: DerivedFees = await this._api.derive.balances.fees();

        const estimatedFee = fees.transactionBaseFee.add(fees.transactionByteFee.mul(transactionByteSize));

        return estimatedFee;
    }

    async transfer(account: KeyringPair, recipientAddr: string, amount: BN): Promise<Hash> {
        const txHash = await this._api.tx.balances
            .transfer(recipientAddr, amount)
            .signAndSend(account);
        return txHash;
    }

    // Membership terms
    async getActiveMembershipTermsIds(): Promise<Vec<PaidTermId>> {
        return <Vec<PaidTermId>> await this._api.query.members.activePaidMembershipTerms();
    }

    async getMebershipTermsDetails(termsId: PaidTermId): Promise<PaidMembershipTerms | null> {
        const res = <Option<PaidMembershipTerms>> (await this._api.query.members.paidMembershipTermsById(termsId));

        return res.unwrapOr(null);
    }

    // It returns details of first terms from array of currently active terms ids
    // (just like in Pioneer)
    async getCurrentMembershipTerms(): Promise<PaidMembershipTerms | null> {
        const activeTermsIds = await this.getActiveMembershipTermsIds();
        if (!activeTermsIds.length) {
            return null
        }

        return await this.getMebershipTermsDetails(activeTermsIds[0]);
    }

    // Account memberships
    async getMembershipsByAddress(address: string): Promise<AccountMemberships> {
        const results = await this.queryMultiOnce([
            [ this._api.query.members.memberIdsByRootAccountId, address ],
            [ this._api.query.members.memberIdsByControllerAccountId, address ]
        ]);
        return <AccountMemberships> {
            rootIds: results[0],
            controllerIds: results[1]
        };
    }

    async getMembershipDetails(memberId: MemberId): Promise<Profile | null> {
        // TODO: Cannot use Option<Profile> as it does not satisfy constraint "Codec"
        const profile = <Option<any>> await this._api.query.members.memberProfile(memberId);

        return <Profile | null> profile.unwrapOr(null);
    }

    // It returns details of first membership from array of combined root+controller address membership ids
    // (just like in Pioneer)
    async getCurrentMembershipByAddress(address: string): Promise<Profile | null> {
        const memberships = await this.getMembershipsByAddress(address);
        const combinedMemberships = memberships.rootIds.concat(memberships.controllerIds);
        if (!combinedMemberships.length) {
            return null;
        }
        const [ defaultMembershipId ] = combinedMemberships;

        return await this.getMembershipDetails(defaultMembershipId);
    }
}
