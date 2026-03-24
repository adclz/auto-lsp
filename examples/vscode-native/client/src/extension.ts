import { spawn } from 'node:child_process';
import { text } from 'node:stream/consumers';
import { ExtensionContext, Uri, window, workspace, commands } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, RequestType, Executable } from 'vscode-languageclient/node';


let client: LanguageClient;

export async function activate(context: ExtensionContext) {
    const serverModule = Uri.joinPath(context.extensionUri,
        "..", "..", "target", ...(
            process.platform === "win32" ? ["x86_64-pc-windows-gnu", "release", "vscode-lsp-server.exe"]
                : ["x86_64-unknown-linux-gnu", "release", "vscode-lsp-server"])
    );

    const channel = window.createOutputChannel('Python LSP Server', "log");
    const run: Executable = {
        command: serverModule.fsPath,
        options: { env: process.env },
    };
    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ language: "python" }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.py')
        },
        outputChannel: channel,
    };

    client = new LanguageClient('lspClient', 'LSP Client',
        serverOptions,
        clientOptions
    );

    try {
        await client.start();
    } catch (error) {
        client.error(`Start failed`, error, 'force');
    }
}

export function deactivate(): Thenable<void> | undefined {
    return client.stop();
}