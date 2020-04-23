import AccountsCommandBase from '../../base/AccountsCommandBase';
import { NameValueObj } from '../../Types';
import { displayNameValueTable } from '../../helpers/display';
import { formatBalance } from '@polkadot/util';
import ExitCodes from '../../ExitCodes';

export default class MembershipCurrent extends AccountsCommandBase {
    static description = 'Display information about current paid membership terms';

    async run() {
        const terms = await this.getApi().getCurrentMembershipTerms();
        if (!terms) {
            this.error('No paid membership terms found!', { exit: ExitCodes.NoTermsFound });
        }

        const membershipTermsRows: NameValueObj[] = [
            { name: 'Register fee:', value: formatBalance(terms.fee) },
            { name: 'Terms:', value: terms.text.toString() || '< Terms string is empty >' }
        ];
        displayNameValueTable(membershipTermsRows);
    }
  }
