import { Route } from './types';

import Tokenomics from '@polkadot/joy-tokenomics/index';

export default function create (t: <T = string> (key: string, text: string, options: { ns: string }) => T): Route {
  return {
    Component: Tokenomics,
    display: {
      needsApi: []
    },
    text: t<string>('nav.tokenomics', 'Overview', { ns: 'apps-routing' }),
    icon: 'th',
    name: 'overview'
  };
}
