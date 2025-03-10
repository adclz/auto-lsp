# Configuring a client

## Acknowledgement

Thanks to [`texter`](https://github.com/airblast-dev/texter) crate, the client can send text in any encoding to the server.

`texter` also provides an efficient way to update documents incrementally.

## File extensions

The LSP server must know how each file extensions are associated with a parser.

The client is responsible for sending this information to the server.

Using `VScode` LSP client, this is done via providing `perFileParser` object in the [`initializationOptions`](https://github.com/microsoft/vscode-languageserver-node/blob/d810d51297c667bd3a3f46912eb849055beb8b6b/client/src/common/client.ts#L360) of `LanguageClientOptions`.

```ts
import { LanguageClient, LanguageClientOptions, ServerOptions, RequestType } from 'vscode-languageclient/node';

// We tell the server that .py files are associated with the python parser defined via the configure_parsers! macro.
const initializationOptions = {
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
```

```admonish
You have an example in the [`vscode-wasi`](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi) folder in the `auto-lsp` repository.
```
