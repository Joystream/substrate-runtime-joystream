// Copyright 2017-2020 @polkadot/react-hooks authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { useEffect, useState } from 'react';
import { keyring } from '@polkadot/ui-keyring';

import useIsMountedRef from './useIsMountedRef';

interface UseAccounts {
  allAddresses: string[];
  hasAddresses: boolean;
  isAddress: (address: string) => boolean;
}

export default function useAccounts (): UseAccounts {
  const mountedRef = useIsMountedRef();
  const [state, setState] = useState<UseAccounts>({ allAddresses: [], hasAddresses: false, isAddress: () => false });

  useEffect((): () => void => {
    const subscription = keyring.addresses.subject.subscribe((addresses): void => {
      if (mountedRef.current) {
        const allAddresses = addresses ? Object.keys(addresses) : [];
        const hasAddresses = allAddresses.length !== 0;
        const isAddress = (address: string): boolean => allAddresses.includes(address.toString());

        setState({ allAddresses, hasAddresses, isAddress });
      }
    });

    return (): void => {
      setTimeout(() => subscription.unsubscribe(), 0);
    };
  }, [mountedRef]);

  return state;
}
