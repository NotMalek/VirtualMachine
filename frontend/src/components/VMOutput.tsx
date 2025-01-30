import React from 'react';
import { BlinkingCursor } from './BlinkingCursor';

interface VMOutputProps {
    output: string[];
}

export const VMOutput: React.FC<VMOutputProps> = ({ output }) => {
    return (
        <div className="terminal-window">
            <h2 className="terminal-heading flex items-center gap-2">
                Output
                <span className="text-xs text-green-600">(vm-terminal)</span>
            </h2>
            <div className="terminal-text h-48 overflow-y-auto bg-black/50 font-mono p-2 rounded-sm">
                <div className="flex flex-col space-y-1">
                    {output.length === 0 ? (
                        <div className="text-green-700 flex items-center">
                            <span>No output</span>
                            <BlinkingCursor />
                        </div>
                    ) : (
                        <>
                            {output.map((line, index) => (
                                <div key={index} className="flex">
                  <span className="text-green-600 min-w-[60px]">
                    {`[${index.toString().padStart(3, '0')}]`}
                  </span>
                                    <span className="text-green-400 flex-1 whitespace-pre">
                    {line}
                  </span>
                                </div>
                            ))}
                            <div className="flex items-center">
                <span className="text-green-600 min-w-[60px]">
                  {`[${output.length.toString().padStart(3, '0')}]`}
                </span>
                                <BlinkingCursor />
                            </div>
                        </>
                    )}
                </div>
            </div>
        </div>
    );
};