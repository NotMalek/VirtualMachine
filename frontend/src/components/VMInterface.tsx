import React, { useState, useEffect } from 'react';
import { VMStack } from './VMStack';
import { VMMemory } from './VMMemory';
import { VMInstructions } from './VMInstructions';
import { VMOutput } from './VMOutput';
import { VMControls } from './VMControls';
import { vmService } from '../services/vmService';
import type { VMState } from '../types/vm';

export default function VMInterface() {
    const [vmState, setVmState] = useState<VMState>({
        stack: [],
        memory: {},
        programCounter: 0,
        instructions: [],
        output: []
    });

    const [isRunning, setIsRunning] = useState(false);
    const [isProgramComplete, setIsProgramComplete] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [runInterval, setRunInterval] = useState<NodeJS.Timeout | null>(null);

    const handleStep = async () => {
        if (isProgramComplete) {
            setError("Program has completed. Reset or load a new program.");
            return;
        }

        try {
            setError(null);
            const newState = await vmService.step();
            setVmState(newState);

            // Check if program counter is at or past the last instruction
            if (newState.programCounter >= newState.instructions.length) {
                setIsProgramComplete(true);
                setIsRunning(false);
                if (runInterval) {
                    clearInterval(runInterval);
                    setRunInterval(null);
                }
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Failed to step VM';
            setError(errorMessage);
            setIsRunning(false);
            if (runInterval) {
                clearInterval(runInterval);
                setRunInterval(null);
            }
        }
    };

    const handleRun = () => {
        if (isProgramComplete) {
            setError("Program has completed. Reset or load a new program.");
            return;
        }

        setIsRunning(true);
        setError(null);

        const interval = setInterval(async () => {
            try {
                const newState = await vmService.step();
                setVmState(newState);

                if (newState.programCounter >= newState.instructions.length) {
                    setIsProgramComplete(true);
                    clearInterval(interval);
                    setRunInterval(null);
                    setIsRunning(false);
                }
            } catch (error) {
                const errorMessage = error instanceof Error ? error.message : 'Failed to run VM';
                setError(errorMessage);
                clearInterval(interval);
                setRunInterval(null);
                setIsRunning(false);
            }
        }, 100);

        setRunInterval(interval);
    };

    const handleReset = async () => {
        if (runInterval) {
            clearInterval(runInterval);
            setRunInterval(null);
        }
        setIsRunning(false);
        setIsProgramComplete(false);
        setError(null);

        try {
            const newState = await vmService.reset();
            setVmState(newState);
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Failed to reset VM';
            setError(errorMessage);
        }
    };

    const handleLoadProgram = async (code: string) => {
        try {
            setError(null);
            setIsProgramComplete(false);
            const newState = await vmService.loadProgram(code);
            setVmState(newState);
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Failed to load program';
            setError(errorMessage);
        }
    };

    return (
        <div className="min-h-screen bg-black p-6 font-mono">
            {error && (
                <div className="mb-6 p-4 border border-red-500 bg-red-900/20 text-red-500">
                    {error}
                </div>
            )}

            {isProgramComplete && (
                <div className="mb-6 p-4 border border-green-500 bg-green-900/20 text-green-500">
                    Program completed successfully
                </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-6">
                    <VMStack stack={vmState.stack} />
                    <VMMemory memory={vmState.memory} />
                </div>

                <div className="space-y-6">
                    <VMInstructions
                        instructions={vmState.instructions}
                        programCounter={vmState.programCounter}
                    />
                    <VMOutput output={vmState.output} />
                </div>
            </div>

            <div className="mt-6">
                <VMControls
                    onStep={handleStep}
                    onRun={handleRun}
                    onReset={handleReset}
                    onLoad={handleLoadProgram}
                    isRunning={isRunning}
                    isProgramComplete={isProgramComplete}
                />
            </div>
        </div>
    );
}