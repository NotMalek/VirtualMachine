import { loader } from '@monaco-editor/react';

export const configureMonaco = () => {
    loader.init().then(monaco => {
        monaco.languages.register({ id: 'vm-assembly' });

        monaco.languages.setMonarchTokensProvider('vm-assembly', {
            tokenizer: {
                root: [
                    // Comments
                    [/\/\/.*$/, 'comment'],

                    // Instructions
                    [/\b(PUSH|POP|DUP|SWAP|ADD|SUB|MUL|DIV|STORE|LOAD|JMP|JMPZ|JMPNZ|PRINT|PRINTCHAR|PRINTSTR|NEWARRAY|ARRAYGET|ARRAYSET|ARRAYLEN|FREEARR|NEWSTRING|STRCAT|STRLEN|FREESTR|HALT)\b/, 'keyword'],

                    // Numbers
                    [/\b\d+\b/, 'number'],

                    // Strings
                    [/"[^"]*"/, 'string'],

                    // Labels
                    [/[a-zA-Z_][a-zA-Z0-9_]*:/, 'type'],

                    // Variables
                    [/[a-zA-Z_][a-zA-Z0-9_]*/, 'identifier'],
                ]
            }
        });

        monaco.editor.defineTheme('vm-dark', {
            base: 'vs-dark',
            inherit: true,
            rules: [
                { token: 'keyword', foreground: '00ff00', fontStyle: 'bold' },
                { token: 'number', foreground: '00ffff' },
                { token: 'string', foreground: 'ffd700' },
                { token: 'comment', foreground: '808080', fontStyle: 'italic' },
                { token: 'type', foreground: 'ff8c00' },
                { token: 'identifier', foreground: 'ffffff' }
            ],
            colors: {
                'editor.background': '#000000',
                'editor.foreground': '#00ff00',
                'editor.lineHighlightBackground': '#003300',
                'editorCursor.foreground': '#00ff00',
                'editor.selectionBackground': '#005500'
            }
        });
    });
};