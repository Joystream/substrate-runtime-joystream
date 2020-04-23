import AccountsCommandBase from '../../base/AccountsCommandBase';
import { flags } from '@oclif/command';
import { IFlag } from '@oclif/command/lib/flags';

// Command flags type
type ApiInspectFlags = {
    handle: string,
    avatarUri: string,
    about: string
};

export default class MembershipEdit extends AccountsCommandBase {
    static description = 'Edit current member profile.';

    static flags: { [T in keyof ApiInspectFlags]: IFlag<any> } = {
        handle: flags.string({
            char: 'h',
            description: 'Updated profile handle',
        }),
        avatarUri: flags.string({
            char: 'a',
            description: 'Updated avatar uri'
        }),
        about: flags.string({
            char: 'A',
            description: 'Updated "About" description',
            dependsOn: ['module'],
        })
    };

    async run() {
        this.log('To be implemented...');
    }
}
