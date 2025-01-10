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

	const channel = window.createOutputChannel('LSP WASM Server', "log");
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
			"st": "iec-61131-2"
		}
	}

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ language: 'st' }],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/*.st')
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

	/*type CountFileParams = { folder: string };
	const CountFilesRequest = new RequestType<CountFileParams, number, void>('wasm-language-server/countFiles');
	context.subscriptions.push(commands.registerCommand('vscode-samples.wasm-language-server.countFiles', async () => {
		// We assume we do have a folder.
		const folder = workspace.workspaceFolders![0].uri;
		// We need to convert the folder URI to a URI that maps to the mounted WASI file system. This is something
		// @vscode/wasm-wasi-lsp does for us.
		const result = await client.sendRequest(CountFilesRequest, { folder: client.code2ProtocolConverter.asUri(folder) });
		window.showInformationMessage(`The workspace contains ${result} files.`);
	}));/*/
}

export function deactivate() {
	return client.stop();
}