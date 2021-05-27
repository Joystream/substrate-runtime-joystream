import React from 'react';
import { Table } from 'semantic-ui-react';
import { ProposalStatisticsData, ProposalStatistics } from '@polkadot/joy-utils/src/types/tokenomics';

const ProposalStatisticsTableRow: React.FC<{ item: string, all: string, text: string, spending: string, workingGroups: string, networkChanges: string }> = ({
  item,
  all,
  text,
  spending,
  workingGroups,
  networkChanges
}) => {
  return (
    <Table.Row>
      <Table.Cell>{item}</Table.Cell>
      <Table.Cell>{all}</Table.Cell>
      <Table.Cell>{text}</Table.Cell>
      <Table.Cell>{spending}</Table.Cell>
      <Table.Cell>{workingGroups}</Table.Cell>
      <Table.Cell>{networkChanges}</Table.Cell>
    </Table.Row>
  );
};

const ProposalStatisticsTable: React.FC<{ title: string, data?: ProposalStatisticsData }> = ({ title, data }) => {
  const parseData = (proposalType: keyof ProposalStatisticsData, proposalStatistics: keyof ProposalStatistics) => {
    return data ? `${data[proposalType][proposalStatistics]}` : 'Loading..';
  };

  return (
    <Table style={{ marginBottom: '1.5rem' }} celled>
      <Table.Header>
        <Table.Row>
          <Table.HeaderCell width={3}>{title}</Table.HeaderCell>
          <Table.HeaderCell width={2}>All</Table.HeaderCell>
          <Table.HeaderCell width={2}>Text</Table.HeaderCell>
          <Table.HeaderCell width={2}>Spending</Table.HeaderCell>
          <Table.HeaderCell width={2}>Working Groups</Table.HeaderCell>
          <Table.HeaderCell width={2}>Network Changes</Table.HeaderCell>
        </Table.Row>
      </Table.Header>

      <Table.Body>
        <ProposalStatisticsTableRow
          item='All Proposals'
          all={parseData('all', 'all')}
          text={parseData('text', 'all')}
          spending={parseData('spending', 'all')}
          workingGroups={parseData('workingGroups', 'all')}
          networkChanges={parseData('networkChanges', 'all')}
        />
        <ProposalStatisticsTableRow
          item='Active Proposals'
          all={parseData('all', 'Active')}
          text={parseData('text', 'Active')}
          spending={parseData('spending', 'Active')}
          workingGroups={parseData('workingGroups', 'Active')}
          networkChanges={parseData('networkChanges', 'Active')}
        />
        <ProposalStatisticsTableRow
          item='Approved Proposals'
          all={parseData('all', 'Approved')}
          text={parseData('text', 'Approved')}
          spending={parseData('spending', 'Approved')}
          workingGroups={parseData('workingGroups', 'Approved')}
          networkChanges={parseData('networkChanges', 'Approved')}
        />
        <ProposalStatisticsTableRow
          item='Rejected Proposals'
          all={parseData('all', 'Rejected')}
          text={parseData('text', 'Rejected')}
          spending={parseData('spending', 'Rejected')}
          workingGroups={parseData('workingGroups', 'Rejected')}
          networkChanges={parseData('networkChanges', 'Rejected')}
        />
        <ProposalStatisticsTableRow
          item='Expired Proposals'
          all={parseData('all', 'Expired')}
          text={parseData('text', 'Expired')}
          spending={parseData('spending', 'Expired')}
          workingGroups={parseData('workingGroups', 'Expired')}
          networkChanges={parseData('networkChanges', 'Expired')}
        />
        <ProposalStatisticsTableRow
          item='Slashed Proposals'
          all={parseData('all', 'Slashed')}
          text={parseData('text', 'Slashed')}
          spending={parseData('spending', 'Slashed')}
          workingGroups={parseData('workingGroups', 'Slashed')}
          networkChanges={parseData('networkChanges', 'Slashed')}
        />
        <ProposalStatisticsTableRow
          item='Cancelled Proposals'
          all={parseData('all', 'Canceled')}
          text={parseData('text', 'Canceled')}
          spending={parseData('spending', 'Canceled')}
          workingGroups={parseData('workingGroups', 'Canceled')}
          networkChanges={parseData('networkChanges', 'Canceled')}
        />
      </Table.Body>
    </Table>
  );
};

export default ProposalStatisticsTable;
