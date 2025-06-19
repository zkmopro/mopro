import React from 'react';

interface ChallengesProps {}

const Challenges: React.FC<ChallengesProps> = () => {
  return (
    <div className="challenges-container" style={{
      maxWidth: '800px',
      margin: '2rem auto',
      padding: '2rem',
      backgroundColor: '#1a1a1a',
      borderRadius: '12px',
      boxShadow: '0 4px 6px rgba(0,0,0,0.2)'
    }}>
      <h3 style={{
        fontSize: '1.8rem',
        color: '#ffffff',
        marginBottom: '1.5rem',
        borderBottom: '2px solid #333',
        paddingBottom: '0.5rem'
      }}>
        ⚙️ <strong>Key Developer Challenges for ZK app development</strong>
      </h3>
      
      <ul style={{
        listStyle: 'none',
        padding: 0,
        margin: 0
      }}>
        <li style={{
          marginBottom: '1.5rem',
          padding: '1.2rem',
          backgroundColor: '#2d2d2d',
          borderRadius: '8px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
          transition: 'transform 0.2s ease-in-out',
          ':hover': {
            transform: 'translateY(-2px)'
          }
        }}>
          <strong style={{
            display: 'block',
            fontSize: '1.2rem',
            color: '#ffffff',
            marginBottom: '0.5rem'
          }}>High Integration Complexity</strong>
          <p style={{
            color: '#b3b3b3',
            margin: 0,
            lineHeight: '1.5'
          }}>Time-consuming native setup and platform-specific headaches</p>
        </li>

        <li style={{
          marginBottom: '1.5rem',
          padding: '1.2rem',
          backgroundColor: '#2d2d2d',
          borderRadius: '8px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
          transition: 'transform 0.2s ease-in-out',
          ':hover': {
            transform: 'translateY(-2px)'
          }
        }}>
          <strong style={{
            display: 'block',
            fontSize: '1.2rem',
            color: '#ffffff',
            marginBottom: '0.5rem'
          }}>Messy multi-language code</strong>
          <p style={{
            color: '#b3b3b3',
            margin: 0,
            lineHeight: '1.5'
          }}>Previous methods like Rapidsnark/Witnesscal required writing native code in C++, Java, and Objective-C, significantly slowing down development.</p>
        </li>

        <li style={{
          marginBottom: '1.5rem',
          padding: '1.2rem',
          backgroundColor: '#2d2d2d',
          borderRadius: '8px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
          transition: 'transform 0.2s ease-in-out',
          ':hover': {
            transform: 'translateY(-2px)'
          }
        }}>
          <strong style={{
            display: 'block',
            fontSize: '1.2rem',
            color: '#ffffff',
            marginBottom: '0.5rem'
          }}>Poor Developer UX</strong>
          <p style={{
            color: '#b3b3b3',
            margin: 0,
            lineHeight: '1.5'
          }}>Developers historically spent more time making circuits run on mobile than building the circuits themselves.</p>
        </li>

        <li style={{
          marginBottom: '1.5rem',
          padding: '1.2rem',
          backgroundColor: '#2d2d2d',
          borderRadius: '8px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
          transition: 'transform 0.2s ease-in-out',
          ':hover': {
            transform: 'translateY(-2px)'
          }
        }}>
          <strong style={{
            display: 'block',
            fontSize: '1.2rem',
            color: '#ffffff',
            marginBottom: '0.5rem'
          }}>Low Portability</strong>
          <p style={{
            color: '#b3b3b3',
            margin: 0,
            lineHeight: '1.5'
          }}>Most ZKP tools weren't built with mobile platforms in mind</p>
        </li>

        <li style={{
          padding: '1.2rem',
          backgroundColor: '#2d2d2d',
          borderRadius: '8px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
          transition: 'transform 0.2s ease-in-out',
          ':hover': {
            transform: 'translateY(-2px)'
          }
        }}>
          <strong style={{
            display: 'block',
            fontSize: '1.2rem',
            color: '#ffffff'
          }}>No GPU optimization</strong>
        </li>
      </ul>
    </div>
  );
};

export default Challenges;
