import React, { useEffect } from 'react';

interface TweetProps {
  tweetId: string;
  width?: string;
}

const Tweet: React.FC<TweetProps> = ({ tweetId, width = '550px' }) => {
  useEffect(() => {
    // Load Twitter widgets script if not already loaded
    if (!(window as any).twttr) {
      const script = document.createElement('script');
      script.src = 'https://platform.twitter.com/widgets.js';
      script.async = true;
      script.charset = 'utf-8';
      document.body.appendChild(script);
    } else {
      // If script is already loaded, reload widgets
      (window as any).twttr.widgets.load();
    }
  }, []);

  return (
    <div style={{ display: 'flex', justifyContent: 'center', margin: '2rem 0' }}>
      <blockquote className="twitter-tweet" data-width={width}>
        <a href={`https://twitter.com/x/status/${tweetId}`}>Loading tweet...</a>
      </blockquote>
    </div>
  );
};

export default Tweet;
