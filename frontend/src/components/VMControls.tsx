import React, { useState } from 'react';
import { VMCodeEditor } from './VMCodeEditor';

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
                                                          isRunning
                                                      }) => {
    const [code, setCode] = useState('');

    const handleLoadClick = () => {
        onLoad(code);
    };

    return (
        <div className="terminal-window">
            <h2 className="terminal-heading">Controls</h2>
            <div className="space-y-4">
                <div className="flex gap-4">
                    <button
                        className="terminal-button"
                        onClick={onStep}
                        disabled={isRunning}
                    >
                        Step
                    </button>
                    <button
                        className="terminal-button"
                        onClick={onRun}
                        disabled={isRunning}
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
        </div>
    );
};