// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import { cpSync } from 'fs';
import { type } from 'os';
import { format } from 'path';
import * as vscode from 'vscode';
import { commands, workspace, ExtensionContext, window, Uri } from 'vscode';
import * as path from 'path';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';
import { start } from 'repl';

let client: LanguageClient;

function startServer(context: vscode.ExtensionContext) {
	let verylaLsIntegrated = context.asAbsolutePath(path.join('bin', 'veryla-ls'));

	let verylaLsBinaryPath: string | undefined = workspace.getConfiguration("vscode-veryla").get("verylaLsBinary.path");
	if (typeof verylaLsBinaryPath === "undefined") {
		verylaLsBinaryPath = verylaLsIntegrated;
	} else if (verylaLsBinaryPath === null) {
		verylaLsBinaryPath = verylaLsIntegrated;
	}

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	let serverOptions: ServerOptions = {
		run: {command: verylaLsBinaryPath},
		debug: {command: verylaLsBinaryPath},
	};

	// Options to control the language client
	let clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'veryla' }],
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'veryla-ls',
		'Veryla language server',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
}

function stopServer(): Thenable<void> {
	if (!client) {
		return Promise.resolve();
	}
	return client.stop();
}

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "vscode-veryla" is now active!');

	context.subscriptions.push(
		commands.registerCommand("vscode-veryla.restartServer", () => {
			stopServer().then(function () {startServer(context);}, startServer);
		})
	);

	startServer(context);
}

// This method is called when your extension is deactivated
export function deactivate(): Thenable<void> {
	return stopServer();
}
