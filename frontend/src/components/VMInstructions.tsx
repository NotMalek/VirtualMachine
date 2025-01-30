import React from 'react';
import type { Instruction } from '../types/vm';

interface VMInstructionsProps {
    instructions: Instruction[];
    programCounter: number;
}

export const VMInstructions: React.FC<VMInstructionsProps> = ({
                                                                  instructions,
                                                                  programCounter
                                                              }) => {
    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Instructions</h2>
            <div className="terminal-text h-48 overflow-y-auto">
                {instructions.map((instr, index) => (
                    <div
                        key={index}
                        className={`font-mono ${index === programCounter ? 'bg-green-900' : ''}`}
                    >
                        {`${index}: ${instr.type}${instr.value !== undefined ? ` ${instr.value}` : ''}`}
                    </div>
                ))}
            </div>
        </div>
    );
};