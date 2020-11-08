import React from 'react';
import styled from 'styled-components';

const RoleCardContentCell: React.FC<{ className?: string; title: string; data: string }> = ({
  className,
  title,
  data
}) => {
  return (
    <div style={{ padding: '0.2rem' }} className={className}>
      <p style={{ textAlign: 'center' }}>{title}</p>
      <h2 style={{ textAlign: 'center' }}>{data}</h2>
    </div>
  );
};

const StyledRoleCardContentCell = styled(RoleCardContentCell)`
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-evenly;
    border-left: 1px solid gray;
    h2{
        margin:0;
    }
    @media (max-width: 700px){
        border-left: 0;
    }
`;

const RoleCardContent: React.FC<{ className?: string, cellData: [string, string][] }> = ({ className, cellData }) => {
  return (
    <div className={className}>
      {cellData.map(([key, value], index: number) => <StyledRoleCardContentCell key={index} title={key} data={value} />)}
    </div>
  );
};

export default RoleCardContent;
