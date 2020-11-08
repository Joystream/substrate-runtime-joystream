import React from 'react';
import styled from 'styled-components';
import GetStarted from './GetStarted';
import Roles from './Roles';

const Title = styled('h2')`
  border-bottom: 1px solid #ddd;
  margin: 0 0 2rem 0;
`;

const Landing = () => {
  return (
    <>
      <Title>Get Started</Title>
      <GetStarted />
      <Title>Roles</Title>
      <Roles />
    </>
  );
};

export default Landing;
