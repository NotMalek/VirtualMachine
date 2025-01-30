import type { VMState } from '../types/vm';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001/api';

export class VMService {
    async loadProgram(code: string): Promise<VMState> {
        try {
            const response = await fetch(`${API_BASE_URL}/load`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ code }),
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`Failed to load program: ${errorText}`);
            }

            const data = await response.json();
            return this.transformVMState(data);
        } catch (error) {
            console.error('Error in loadProgram:', error);
            throw error;
        }
    }

    async step(): Promise<VMState> {
        try {
            const response = await fetch(`${API_BASE_URL}/step`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                }
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`Step failed: ${errorText}`);
            }

            const data = await response.json();
            return this.transformVMState(data);
        } catch (error) {
            console.error('Error in step:', error);
            throw error;
        }
    }

    async reset(): Promise<VMState> {
        try {
            const response = await fetch(`${API_BASE_URL}/reset`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                }
            });

            if (!response.ok) {
                throw new Error('Failed to reset VM');
            }

            return {
                stack: [],
                memory: {},
                programCounter: 0,
                instructions: [],
                output: [],
            };
        } catch (error) {
            console.error('Error in reset:', error);
            throw error;
        }
    }

    async getState(): Promise<VMState> {
        try {
            const response = await fetch(`${API_BASE_URL}/state`);

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`Failed to get state: ${errorText}`);
            }

            const data = await response.json();
            return this.transformVMState(data);
        } catch (error) {
            console.error('Error in getState:', error);
            throw error;
        }
    }

    private transformVMState(data: any): VMState {
        try {
            // Transform the instruction strings back to proper Instruction objects
            const instructions = data.instructions.map((instrStr: string) => {
                const [type, ...args] = instrStr.split(' ');
                return {
                    type,
                    value: args.length > 0 ? args.join(' ').replace(/"/g, '') : undefined,
                };
            });

            return {
                stack: data.stack || [],
                memory: data.memory || {},
                programCounter: data.program_counter || 0,
                instructions,
                output: data.output || [],
            };
        } catch (error) {
            console.error('Error transforming VM state:', error);
            throw error;
        }
    }
}

export const vmService = new VMService();