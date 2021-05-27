import React from 'react';
import { useTranslation } from './translate';
import { Route, Switch } from 'react-router';
import { Tabs } from '@polkadot/react-components';
import Overview from './Overview';
import Stats from './Stats';
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
              name: 'overview',
              text: t('Tokenomics')
            },
            {
              name: 'stats',
              text: t('Stats')
            }
          ]}
        />
      </header>
      <Switch>
        <Route path={`${basePath}/stats`} component={Stats} />
        <Route component={Overview} />
      </Switch>
    </main>
  );
}

export default App;
