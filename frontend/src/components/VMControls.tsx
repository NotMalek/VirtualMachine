import React, { useState } from 'react';
import { VMCodeEditor } from './VMCodeEditor';
import { VMInstructionsModal } from './VMInstructionsModal';

interface VMControlsProps {
    onStep: () => void;
    onRun: () => void;
    onReset: () => void;
    onLoad: (code: string) => void;
    isRunning: boolean;
    isProgramComplete: boolean;
}

export const VMControls: React.FC<VMControlsProps> = ({
                                                          onStep,
                                                          onRun,
                                                          onReset,
                                                          onLoad,
                                                          isRunning,
                                                          isProgramComplete
                                                      }) => {
    const [code, setCode] = useState('');
    const [isInstructionsOpen, setIsInstructionsOpen] = useState(false);

    const handleLoadClick = () => {
        onLoad(code);
    };

    return (
        <div className="terminal-window">
            <div className="flex justify-between items-center mb-4">
                <h2 className="terminal-heading">Controls</h2>
                <button
                    className="terminal-button text-sm"
                    onClick={() => setIsInstructionsOpen(true)}
                >
                    Instructions Reference
                </button>
            </div>

            <div className="space-y-4">
                <div className="flex gap-4">
                    <button
                        className="terminal-button"
                        onClick={onStep}
                        disabled={isRunning || isProgramComplete}
                    >
                        Step
                    </button>
                    <button
                        className="terminal-button"
                        onClick={onRun}
                        disabled={isRunning || isProgramComplete}
                    >
                        Run
                    </button>
                    <button
                        className="terminal-button"
                        onClick={onReset}
                    >
                        Reset
                    </button>
                </div>

                <VMCodeEditor
                    value={code}
                    onChange={setCode}
                />

                <button
                    className="terminal-button w-full"
                    onClick={handleLoadClick}
                >
                    Load Program
                </button>
            </div>

            <VMInstructionsModal
                isOpen={isInstructionsOpen}
                onClose={() => setIsInstructionsOpen(false)}
            />
        </div>
    );
};