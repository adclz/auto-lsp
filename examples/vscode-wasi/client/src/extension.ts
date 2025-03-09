/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { ExtensionContext, Uri, window, workspace, commands } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, RequestType } from 'vscode-languageclient/node';
import { Wasm, ProcessOptions } from '@vscode/wasm-wasi/v1';
import { createStdioOptions, createUriConverters, startServer } from '@vscode/wasm-wasi-lsp';

let client: LanguageClient;
interface InitializationOptions {
	perFileParser: Record<string, string>;
}

export async function activate(context: ExtensionContext) {
	const wasm: Wasm = await Wasm.load();

	const channel = window.createOutputChannel('Python LSP WASM Server', "log");
	const serverOptions: ServerOptions = async () => {
		const options: ProcessOptions = {
			stdio: createStdioOptions(),
			mountPoints: [
				{ kind: 'workspaceFolder' },
			]
		};
		const filename = Uri.joinPath(context.extensionUri, 'server', 'target', 'wasm32-wasip1-threads', 'release', 'server.wasm');
		const bits = await workspace.fs.readFile(filename);
		const module = await WebAssembly.compile(bits);
		const process = await wasm.createProcess('lsp-server', module, { initial: 160, maximum: 160, shared: true }, options);

		const decoder = new TextDecoder('utf-8');
		process.stderr!.onData((data) => {
			channel.append(decoder.decode(data));
		});

		const server = await startServer(process);
		return server
	};

	const initializationOptions: InitializationOptions = {
		perFileParser: {
			"py": "python"
		}
	}

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ language: 'python' }],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/*.py')
		},
		outputChannel: channel,
		uriConverters: createUriConverters(),
		initializationOptions
	};

	client = new LanguageClient('lspClient', 'LSP Client', serverOptions, clientOptions);
	try {
		await client.start();
	} catch (error) {
		client.error(`Start failed`, error, 'force');
	}
}

export function deactivate() {
	return client.stop();
}