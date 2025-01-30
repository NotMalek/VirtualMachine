import React from 'react';

interface VMOutputProps {
    output: string[];
}

export const VMOutput: React.FC<VMOutputProps> = ({ output }) => {
    console.log("Current output in component:", output); // Debug log

    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Output</h2>
            <div className="terminal-text h-48 overflow-y-auto">
                {output.length === 0 ? (
                    <span className="text-green-700">No output</span>
                ) : (
                    output.map((line, index) => (
                        <div key={index} className="font-mono whitespace-pre">
                            {line}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};