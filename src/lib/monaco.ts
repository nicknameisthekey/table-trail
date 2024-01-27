import * as monaco from 'monaco-editor';

import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
// import sqlWorker from 'monaco-editor/esm/vs/basic-languages/sql/sql.worker?worker';

self.MonacoEnvironment = {
    getWorker: function (_: string, label: string) {
        switch (label) {
            // case 'sql':
            //     return new sqlWorker();
            default:
                return new editorWorker();
        }
    }
};

export default monaco;