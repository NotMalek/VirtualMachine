import React from 'react';
import type { Instruction } from '../types/vm';
import { BlinkingCursor } from './BlinkingCursor';

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
            <h2 className="terminal-heading flex items-center gap-2">
                Instructions
                <span className="text-xs text-green-600">(program)</span>
            </h2>
            <div className="terminal-text h-48 overflow-y-auto bg-black/50 font-mono p-2 rounded-sm">
                {instructions.length === 0 ? (
                    <div className="text-green-700 flex items-center">
                        <span>No instructions loaded</span>
                        <BlinkingCursor />
                    </div>
                ) : (
                    <div className="flex flex-col space-y-1">
                        {instructions.map((instr, index) => (
                            <div
                                key={index}
                                className={`flex ${index === programCounter ? 'bg-green-900/30' : ''}`}
                            >
                <span className="text-green-600 min-w-[60px] select-none">
                  {`[${index.toString().padStart(3, '0')}]`}
                </span>
                                <span className="text-green-400 flex-1">
                  {index === programCounter && <span className="text-yellow-500">→ </span>}
                                    {`${instr.type}${instr.value !== undefined ? ` ${instr.value}` : ''}`}
                                    {index === programCounter && <BlinkingCursor />}
                </span>
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
};