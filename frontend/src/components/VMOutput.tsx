import React from 'react';

interface VMOutputProps {
    output: string[];
}

export const VMOutput: React.FC<VMOutputProps> = ({ output }) => {
    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Output</h2>
            <div className="terminal-text h-48 overflow-y-auto">
                {output.map((line, index) => (
                    <div key={index} className="font-mono">
                        {line}
                    </div>
                ))}
            </div>
        </div>
    );
};