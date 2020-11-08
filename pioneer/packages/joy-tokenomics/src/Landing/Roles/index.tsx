import React from 'react';
import styled from 'styled-components';
import Role from './Role';
import { Grid, Icon } from 'semantic-ui-react';
import usePromise from '@polkadot/joy-utils/react/hooks/usePromise';
import { useTransport, useMyAccount } from '@polkadot/joy-utils/react/hooks';

const StyledLoading = styled('div')`
  width: 100%;
  display: flex;
  .circle.notched.loading.icon {
    margin: 3rem auto 0 auto !important;
  }
`;

const Roles = () => {
  const transport = useTransport();
  const myAccount = useMyAccount();
  const [landingPageData] = usePromise(() => transport.tokenomics.landingPageData(myAccount.state.address), undefined, []);
  const [councilStageData] = usePromise(() => transport.tokenomics.councilStageData(), undefined, []);
  const [storageProviderOpeningsData] = usePromise(() => transport.tokenomics.openingData('storageWorkingGroup'), undefined, []);
  const [contentCuratorOpeningsData] = usePromise(() => transport.tokenomics.openingData('contentWorkingGroup'), undefined, []);

  return (
    <>
      <p>
        These are the roles you can currently occupy, and earn on-chain rewards for. You can also earn rewards by
        participating in one of our <a href='https://www.joystream.org/testnet/'>Bounties</a>.
      </p>
      {landingPageData ? (
        <Grid>
          <Grid.Row columns={2}>
            <Grid.Column>
              <Role
                roleData={landingPageData.validator}
                title='Validator'
              />
            </Grid.Column>
            <Grid.Column>
              <Role
                roleData={landingPageData.nominator}
                title='Nominator'
              />
            </Grid.Column>
          </Grid.Row>

          <Grid.Row columns={2}>
            <Grid.Column>
              <Role
                roleData={landingPageData.storageProviders}
                title='Storage Providers'
                extraData={storageProviderOpeningsData}
              />
            </Grid.Column>
            <Grid.Column>
              <Role
                roleData={landingPageData.contentCurators}
                title='Content Curators'
                extraData={contentCuratorOpeningsData}
              />
            </Grid.Column>
          </Grid.Row>

          <Grid.Row stretched columns={1}>
            <Grid.Column>
              <Role
                roleData={landingPageData.council}
                title='Council Members'
                extraData={councilStageData}
              />
            </Grid.Column>
          </Grid.Row>
        </Grid>
      ) : (
        <StyledLoading>
          <Icon size='large' name='circle notched' loading />
        </StyledLoading>
      )}
    </>
  );
};

export default Roles;
