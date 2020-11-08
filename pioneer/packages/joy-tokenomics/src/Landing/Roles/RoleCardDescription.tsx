import React from 'react';
import { Card, Button } from 'semantic-ui-react';
import styled from 'styled-components';

const RoleCardDescriptionWrapper = styled(Card.Content)`
    border-top: 0 !important;
    margin-top:0.5rem !important;
    @media (max-width: 700px){
        display:flex;
        flex-direction: column-reverse;
    }
`;

const StyledRoleCardDescriptionButton = styled(Button)`
    float: right !important;
    margin:0 0 0.2rem 0.5rem !important;
    @media (max-width: 700px){
        float: none !important;
        margin-top: 0.5rem !important;
      }
`;

const RoleCardDescription: React.FC<{group: string}> = ({ group }) => {
  const getHelpDeskUrl = () => {
    let route = '';

    switch (group) {
      case 'Validator':
        route = 'validators';
        break;
      case 'Nominator':
        route = 'validators#nominating';
        break;
      case 'Storage Providers':
        route = 'storage-providers';
        break;
      case 'Content Curators':
        route = 'content-curators';
        break;
      case 'Council Members':
        route = 'council-members';
        break;
      default:
        break;
    }

    return `https://github.com/Joystream/helpdesk/tree/master/roles/${route}`;
  };

  return (
    <RoleCardDescriptionWrapper>
      <StyledRoleCardDescriptionButton href={getHelpDeskUrl()} content='Learn more' size='large'/>
      <p>There are many variations of passages of Lorem Ipsum available, but the majority have suffered</p>
    </RoleCardDescriptionWrapper>
  );
};

export default RoleCardDescription;
