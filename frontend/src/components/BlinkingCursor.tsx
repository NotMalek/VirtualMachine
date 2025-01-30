import React from 'react';

export const BlinkingCursor: React.FC = () => {
    return (
        <span className="inline-block w-2 h-4 ml-1 bg-green-500 animate-[blink_1s_infinite]" />
    );
};