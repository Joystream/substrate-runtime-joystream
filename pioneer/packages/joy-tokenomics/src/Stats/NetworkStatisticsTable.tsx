import React from 'react';
import { Table } from 'semantic-ui-react';
import { NetworkStatisticsData } from '@polkadot/joy-utils/src/types/tokenomics';

const NetworkStatisticsTableRow: React.FC<{ item: string; value: string }> = ({ item, value }) => {
  return (
    <Table.Row>
      <Table.Cell>{item}</Table.Cell>
      <Table.Cell>{value}</Table.Cell>
    </Table.Row>
  );
};

const NetworkStatistics: React.FC<{ data?: NetworkStatisticsData }> = ({ data }) => {
  const parseData = (parameter: keyof NetworkStatisticsData) => {
    return data ? `${data[parameter]}` : 'Loading..';
  };

  return (
    <Table style={{ marginBottom: '1.5rem' }} celled>
      <Table.Header>
        <Table.Row>
          <Table.HeaderCell width={10}>Item</Table.HeaderCell>
          <Table.HeaderCell width={2}>Value</Table.HeaderCell>
        </Table.Row>
      </Table.Header>

      <Table.Body>
        <NetworkStatisticsTableRow item={'Block height'} value={data ? `#${data.blockHeight}` : 'Loading...'} />
        <NetworkStatisticsTableRow item={'Members'} value={parseData('numberOfMembers')} />
        <NetworkStatisticsTableRow item={'Content'} value={parseData('content')} />
        <NetworkStatisticsTableRow item={'Channels'} value={parseData('numberOfChannels')} />
        <NetworkStatisticsTableRow item={'All Proposals'} value={data ? `${data.proposalCount}+(${data.historicalProposals})` : 'Loading...'} />
        <NetworkStatisticsTableRow item={'Forum Categories'} value={parseData('numberOfForumCategories')} />
        <NetworkStatisticsTableRow item={'Forum Posts'} value={parseData('numberOfForumPosts')} />
        <NetworkStatisticsTableRow item={'Council Mint Capacity'} value={parseData('councilMintCapacity')} />
        <NetworkStatisticsTableRow item={'Council Mint Spent'} value={parseData('councilMintSpent')} />
        <NetworkStatisticsTableRow item={'Storage Mint Capacity'} value={parseData('storageProviderMintCapacity')} />
        <NetworkStatisticsTableRow item={'Storage Mint Spent'} value={parseData('storageProviderMintSpent')} />
        <NetworkStatisticsTableRow item={'Curator Mint Capacity'} value={parseData('contentCuratorMintCapacity')} />
        <NetworkStatisticsTableRow item={'Curator Mint Spent'} value={parseData('contentCuratorMintSpent')} />
      </Table.Body>
    </Table>
  );
};

export default NetworkStatistics;
