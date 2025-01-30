import React, { useEffect } from 'react';
import Editor from '@monaco-editor/react';

interface VMCodeEditorProps {
    value: string;
    onChange: (value: string) => void;
}

const SAMPLE_PROGRAMS = {
    basic: `// Basic arithmetic
PUSH 10
PUSH 5
ADD
PRINT`,

    array: `// Array operations
PUSH 3       // array size
NEWARRAY

// Store some values
DUP
PUSH 0
PUSH 100
ARRAYSET

DUP
PUSH 1
PUSH 200
ARRAYSET

// Read and print a value
DUP
PUSH 1
ARRAYGET
PRINT`,

    strings: `// String operations
NEWSTRING "Hello, "
NEWSTRING "VM!"
STRINGCONCAT
PRINT`
} as const;

// Type for our sample program keys
type SampleProgramKey = keyof typeof SAMPLE_PROGRAMS;

export const VMCodeEditor: React.FC<VMCodeEditorProps> = ({ value, onChange }) => {
    // Function to load a sample program
    const loadSampleProgram = (program: SampleProgramKey) => {
        onChange(SAMPLE_PROGRAMS[program]);
    };

    return (
        <div className="space-y-4">
            <div className="flex gap-2">
                <button
                    className="terminal-button text-sm"
                    onClick={() => loadSampleProgram('basic')}
                >
                    Load Basic Example
                </button>
                <button
                    className="terminal-button text-sm"
                    onClick={() => loadSampleProgram('array')}
                >
                    Load Array Example
                </button>
                <button
                    className="terminal-button text-sm"
                    onClick={() => loadSampleProgram('strings')}
                >
                    Load String Example
                </button>
            </div>

            <div className="border border-green-500">
                <Editor
                    height="300px"
                    defaultLanguage="cpp"
                    theme="vs-dark"
                    value={value}
                    onChange={(value) => onChange(value || '')}
                    options={{
                        minimap: { enabled: false },
                        fontSize: 14,
                        fontFamily: 'Courier New',
                        scrollBeyondLastLine: false,
                        lineNumbers: 'on',
                        renderLineHighlight: 'all',
                        wordWrap: 'on',
                    }}
                />
            </div>
        </div>
    );
};