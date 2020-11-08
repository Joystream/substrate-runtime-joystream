import React from 'react';
import styled from 'styled-components';
import usePromise from '@polkadot/joy-utils/react/hooks/usePromise';
import { useTransport } from '@polkadot/joy-utils/react/hooks';

const Subtitle = styled('h3')`
    font-weight: 600;
    color: rgba(0,0,0, 0.65);
    margin-bottom: 1rem;
`;

const GetStarted = () => {
  const transport = useTransport();
  const [data] = usePromise(() => transport.proposals.proposalsBatch('Active'), undefined, []);

  return <>
    <p>If you&apos;re new to Joystream, go to our <a href='https://github.com/Joystream/helpdesk/'>Helpdesk</a> to learn what you can do, and how to get started. If you&apos;re not there already, join our <a href='https://t.me/joinchat/CNyeUxHD9H56m3e_44hXIA'>telegram group</a>, get some tokens, and join our community!</p>
    <Subtitle>Membership</Subtitle>
    <p>Most actions on the platform requires you to become a <a href='/#/members/edit'>Member</a>.</p>
    <Subtitle>Council</Subtitle>
    <p>As &quot;The video platfrom DAO&quot;, Joystreams day to day operations are controlled by the <a href='/#/council/members'>Council</a> through a Proposal system.</p>
    <Subtitle>Proposals</Subtitle>
    <p>There are currently {data?.proposals.length || 'no'} <a href='/#/proposals'>Proposals</a> open for voting.</p>
    {data?.proposals.length
      ? <ul>
        {data.proposals.map((proposal, index) => <li key={index}><a href={`/#/proposals/${proposal.id.toString()}`}>{proposal.title}</a> opened by <a href={`/#/members/${proposal.proposer.handle}`}>{proposal.proposer.handle}</a></li>)}
      </ul> : null
    }
    <Subtitle>Roles</Subtitle>
    <p>In addition to being a Council Member, the other &quot;official&quot; roles are:</p>
    <ul>
      <li><a href='/#/staking'>Validator</a> (and Nominators)</li>
      <li><a href='/#/working-groups'>Storage Provider</a></li>
      <li><a href='/#/working-groups'>Content Curator</a></li>
    </ul>
    <Subtitle>Media</Subtitle>
    <p>Go <a href='/#/media'>here</a> to..</p>
    <p>Although there are no on-chain rewards for doing so, you can also create a <a href='/#/media/channels/new'>Channel</a> and upload videos, just make sure they are in line with the platforms <a href='https://www.joystream.org/privacy-policy/'>Terms of Service</a>.</p>
  </>;
};

export default GetStarted;
