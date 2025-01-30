import React, { useState } from 'react';
import Editor from '@monaco-editor/react';
import { examplePrograms } from '../data/examplePrograms';

interface VMCodeEditorProps {
    value: string;
    onChange: (value: string) => void;
}

export const VMCodeEditor: React.FC<VMCodeEditorProps> = ({ value, onChange }) => {
    const [selectedCategory, setSelectedCategory] = useState<string>('basic');

    const loadSampleProgram = (program: keyof typeof examplePrograms) => {
        onChange(examplePrograms[program]);
        setSelectedCategory(program);
    };

    return (
        <div className="space-y-4">
            <div className="flex flex-wrap gap-2">
                {Object.entries(examplePrograms).map(([key, _]) => (
                    <button
                        key={key}
                        className={`terminal-button text-sm px-3 py-1
              ${selectedCategory === key ? 'bg-green-700' : ''}`}
                        onClick={() => loadSampleProgram(key as keyof typeof examplePrograms)}
                    >
                        {key.charAt(0).toUpperCase() + key.slice(1)}
                    </button>
                ))}
            </div>

            <div className="border border-green-500 relative">
                <div className="absolute top-0 right-0 z-10 p-1">
          <span className="text-xs text-green-600 bg-black px-2 py-1 rounded-bl">
            Assembly Editor
          </span>
                </div>
                <Editor
                    height="300px"
                    defaultLanguage="cpp" // Close enough for our assembly syntax
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
                        glyphMargin: true,
                        folding: true,
                        lineDecorationsWidth: 10,
                        lineNumbersMinChars: 3,
                    }}
                />
            </div>
        </div>
    );
};