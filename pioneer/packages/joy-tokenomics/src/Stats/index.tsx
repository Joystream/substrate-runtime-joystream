import React from 'react';
import { useTransport } from '@polkadot/joy-utils/react/hooks';
import usePromise from '@polkadot/joy-utils/react/hooks/usePromise';
import styled from 'styled-components';
import NetworkStatisticsTable from './NetworkStatisticsTable';
import ProposalStatisticsTable from './ProposalStatisticsTable';

const Title = styled('h2')`
  border-bottom: 1px solid #ddd;
  margin: 0 0 2rem 0;
`;

const Stats = () => {
  const transport = useTransport();
  const [networkStatisticsData] = usePromise(() => transport.tokenomics.networkStatistics(), undefined, []);
  const [proposalStatisticsData] = usePromise(() => transport.tokenomics.proposalStatistics(), undefined, []);
  const historicalProposals = transport.tokenomics.historicalProposalStatistics();

  return (
    <>
      <Title> Network Statistics </Title>
      <NetworkStatisticsTable data={networkStatisticsData}/>
      <Title>Proposal Statistics</Title>
      <ProposalStatisticsTable title='Active Testnet' data={proposalStatisticsData}/>
      <ProposalStatisticsTable title='Historical Testnets' data={historicalProposals} />
    </>
  );
};

export default Stats;
