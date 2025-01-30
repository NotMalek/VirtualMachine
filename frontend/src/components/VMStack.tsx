import React from 'react';

interface VMStackProps {
    stack: number[];
}

export const VMStack: React.FC<VMStackProps> = ({ stack }) => {
    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Stack</h2>
            <div className="terminal-text h-48 overflow-y-auto">
                {stack.length === 0 ? (
                    <span className="text-green-700">Empty</span>
                ) : (
                    stack.map((value, index) => (
                        <div key={index} className="font-mono">
                            {`${stack.length - 1 - index}: ${value}`}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};