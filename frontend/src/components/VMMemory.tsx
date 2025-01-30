import React from 'react';

interface VMMemoryProps {
    memory: Record<string, number>;
}

export const VMMemory: React.FC<VMMemoryProps> = ({ memory }) => {
    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Memory</h2>
            <div className="terminal-text h-48 overflow-y-auto">
                {Object.keys(memory).length === 0 ? (
                    <span className="text-green-700">Empty</span>
                ) : (
                    Object.entries(memory).map(([key, value]) => (
                        <div key={key} className="font-mono">
                            {`${key}: ${value}`}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};