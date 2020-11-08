import React from 'react';
import { useTranslation } from './translate';
import { Route, Switch } from 'react-router';
import { Tabs } from '@polkadot/react-components';
import Overview from './Overview';
import Landing from './Landing';
import { AppProps } from '@polkadot/react-components/types';

type Props = AppProps

function App ({ basePath }: Props): React.ReactElement<Props> {
  const { t } = useTranslation();

  return (
    <main>
      <header>
        <Tabs
          basePath={basePath}
          items={[
            {
              isRoot: true,
              name: 'landing',
              text: t('Landing')
            },
            {
              name: 'tokenomics',
              text: t('Tokenomics')
            }
          ]}
        />
      </header>
      <Switch>
        <Route path={`${basePath}/tokenomics`} component={Overview} />
        <Route component={Landing} />
      </Switch>
    </main>
  );
}

export default App;
