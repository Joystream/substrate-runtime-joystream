import React from 'react';
import styled from 'styled-components';
import { Button } from 'semantic-ui-react';

const WorkingGroupExtraButton = styled(Button)`
    margin-top: 1.5rem !important;
    margin-left: 0.5rem !important;
    @media (max-width: 700px){
        width: 100% !important;
        margin-left: 0rem !important;
    }
`;

export const WorkingGroupExtra: React.FC<{href: string}> = ({ href }) => {
  return (
    <WorkingGroupExtraButton href={href}>Learn More and Apply</WorkingGroupExtraButton>
  );
};

const CouncilExtraWrapper = styled('div')`
    display:flex;
    justify-content: space-evenly;
    margin-top: 1rem;
    text-align:center;
    @media (max-width: 700px){
        flex-direction: column;
        margin-top:0rem;
    }
`;

const CouncilExtraCell = styled('div')`
    display: flex;
    align-items: center;
    justify-content: space-evenly;
    flex-direction: column;
    margin:0 1rem;
    h2{
        margin: 0;
    }
    @media (max-width: 700px){
        margin-top:1rem;
        .ui.button{
            width: 100%;
        }
    }
`;

export const CouncilExtra: React.FC<{termReward: string, termMaxBonus: string, text: string, buttonText: string, buttonHref: string}> = ({ termReward, termMaxBonus, text, buttonText, buttonHref }) => {
  return (
    <CouncilExtraWrapper>
      <CouncilExtraCell>
        <p>TERM REWARD</p>
        <h2>{termReward}</h2>
      </CouncilExtraCell>
      <CouncilExtraCell>
        <p>TERM MAX BONUS</p>
        <h2>{termMaxBonus}</h2>
      </CouncilExtraCell>
      <CouncilExtraCell>
        <p dangerouslySetInnerHTML={{ __html: text }}/>
        <Button size='large' href={buttonHref}>{buttonText}</Button>
      </CouncilExtraCell>
    </CouncilExtraWrapper>
  );
};
