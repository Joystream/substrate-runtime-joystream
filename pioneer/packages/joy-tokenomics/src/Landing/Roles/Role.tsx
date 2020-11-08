import React from 'react';
import styled from 'styled-components';
import { Card } from 'semantic-ui-react';
import RoleCardContent from './RoleCardContent';
import RoleCardDescription from './RoleCardDescription';
import { WorkingGroupExtra, CouncilExtra } from './RoleCardExtra';
import { LandingPageCouncilExtra, LandingPageWorkingGroupExtra } from '../../../../joy-utils/src/types/tokenomics';

const StyledRoleCard = styled(Card)`
  width: 100% !important;
  margin: 0 !important;
  .circle.notched.loading.icon {
    margin: 2rem auto !important;
  }
`;

const StyledRoleCardContent = styled(RoleCardContent)`
  margin-top: 1rem;
  display: grid;
  grid-template-columns: repeat(${(props) => (props ? props.cellData.length : 1)}, 1fr);
  div:first-child {
    border-left: none;
  }
  @media (max-width: 700px) {
    grid-template-columns: none;
    grid-template-rows: repeat(${(props) => (props ? props.cellData.length : 1)}, 1fr);
    grid-row-gap: 1rem;
  }
`;

const StyledRoleCardHeader = styled(Card.Content)`
  background-color: rgba(0, 0, 0, 0.05) !important;
`;

const Role: React.FC<{
  title: string
  roleData: [string, string][]
  extraData?: LandingPageWorkingGroupExtra[] | LandingPageCouncilExtra[]
}> = ({ title, roleData, extraData }) => {
  return (
    <StyledRoleCard>
      <StyledRoleCardHeader textAlign='center' header={title} />
      {extraData && (extraData as any[]).map((data: LandingPageWorkingGroupExtra | LandingPageCouncilExtra, index: number) => (
        <div style={{ margin: '0.5rem 0 1rem', padding: '0.5rem' }} key={index}>
          <h3 style={{ textAlign: 'center', color: data.title.color }}>{data.title.value}</h3>
          <StyledRoleCardContent cellData={data.cellData} />
          {data.type === 'WorkingGroup' ? (
            <WorkingGroupExtra href='/#/working-groups/opportunities' />
          ) : (
            <CouncilExtra
              termReward={data.extra.termReward}
              termMaxBonus={data.extra.termMaxBonus}
              text={data.extra.text}
              buttonText={data.extra.button.text}
              buttonHref={data.extra.button.href}
            />
          )}
        </div>
      ))}
      {extraData?.[0]?.type === 'Council' && (
        <h3 style={{ textAlign: 'center', marginTop: '0', fontStyle: 'italic', fontWeight: 600 }}>Current Council</h3>
      )}
      <StyledRoleCardContent cellData={roleData} />
      <RoleCardDescription group={title}/>
    </StyledRoleCard>
  );
};

export default Role;
