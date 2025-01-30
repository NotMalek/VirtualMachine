import React, { useState } from 'react';
import { instructions } from '../data/instructions';

interface VMInstructionsModalProps {
    isOpen: boolean;
    onClose: () => void;
}

export const VMInstructionsModal: React.FC<VMInstructionsModalProps> = ({
                                                                            isOpen,
                                                                            onClose
                                                                        }) => {
    const [selectedCategory, setSelectedCategory] = useState<string>('All');

    const categories = ['All', ...new Set(instructions.map(i => i.category))];
    const filteredInstructions = selectedCategory === 'All'
        ? instructions
        : instructions.filter(i => i.category === selectedCategory);

    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
            <div className="bg-black border border-green-500 rounded-lg max-w-4xl w-full max-h-[90vh] overflow-hidden">
                <div className="p-4 border-b border-green-500">
                    <div className="flex justify-between items-center">
                        <h2 className="terminal-heading text-2xl">Instructions Reference</h2>
                        <button
                            onClick={onClose}
                            className="terminal-button px-2 py-1"
                        >
                            ✕
                        </button>
                    </div>

                    {/* Category Filter */}
                    <div className="flex gap-2 mt-4 overflow-x-auto pb-2">
                        {categories.map(category => (
                            <button
                                key={category}
                                onClick={() => setSelectedCategory(category)}
                                className={`terminal-button px-3 py-1 text-sm whitespace-nowrap
                  ${selectedCategory === category ? 'bg-green-700' : ''}`}
                            >
                                {category}
                            </button>
                        ))}
                    </div>
                </div>

                <div className="overflow-y-auto max-h-[calc(90vh-120px)] p-4">
                    <div className="grid gap-4">
                        {filteredInstructions.map((instruction, index) => (
                            <div
                                key={index}
                                className="border border-green-500/50 rounded p-4 space-y-2"
                            >
                                <h3 className="text-green-400 font-bold font-mono">{instruction.name}</h3>
                                <p className="text-green-300">{instruction.description}</p>
                                <pre className="bg-green-950/30 p-2 rounded text-green-400 font-mono text-sm">
                  {instruction.example}
                </pre>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
};