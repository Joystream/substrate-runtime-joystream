import AccountsCommandBase from '../../base/AccountsCommandBase';
import { NameValueObj } from '../../Types';
import { displayHeader, displayNameValueTable } from '../../helpers/display';
import moment from 'moment';
import ExitCodes from '../../ExitCodes';
import { Role, ActorInRole } from '@joystream/types/src/members';

export default class MembershipCurrent extends AccountsCommandBase {
    static description = 'Display information about current\'s account memberships';
    static aliases = ['membership:info'];

    async run() {
        const membership = await this.getRequiredMembership(false);

        const roles = membership.roles
            .map((actorInRole: ActorInRole): string | null => {
                const role = <Role | undefined> actorInRole.get('role');
                return role ? role.type : null;
            })
            .filter(r => r !== null);

        const registerDateStr: string = moment(membership.registered_at_time.toNumber()).format('YYYY-MM-DD HH:mm:ss');
        const registeredStr: string = `${registerDateStr} (block #${ membership.registered_at_block.toNumber() })`;

        displayHeader('Profile');
        const membershipDetailsRows: NameValueObj[] = [
            { name: 'Handle:', value: membership.handle.toString() },
            { name: 'Registered:',  value: registeredStr },
            { name: 'Roles:', value: roles.length ? roles.join(', ') : 'None' },
            { name: 'Suspended:', value: membership.suspended.isTrue ? 'Yes' : 'No' },
            { name: 'Avatar uri:', value: membership.avatar_uri.toString() },
            { name: 'About:',  value: membership.about.toString() }
        ];
        displayNameValueTable(membershipDetailsRows);
    }
  }
