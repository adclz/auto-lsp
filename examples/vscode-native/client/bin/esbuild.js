//@ts-check
const esbuild = require('esbuild');

/**
 * @typedef {import('esbuild').BuildOptions} BuildOptions
 */


/** @type BuildOptions */
const sharedDesktopOptions = {
    bundle: true,
    external: ['vscode'],
    target: 'es2020',
    platform: 'node',
    sourcemap: true,
};

/** @type BuildOptions */
const desktopOptions = {
    entryPoints: ['src/extension.ts'],
    outfile: 'dist/desktop/extension.js',
    format: 'cjs',
    ...sharedDesktopOptions,
};

function createContexts() {
    return Promise.all([
        esbuild.context(desktopOptions),
    ]);
}

createContexts().then(contexts => {
    if (process.argv[2] === '--watch') {
        const promises = [];
        for (const context of contexts) {
            promises.push(context.watch());
        }
        return Promise.all(promises).then(() => { return undefined; });
    } else {
        const promises = [];
        for (const context of contexts) {
            promises.push(context.rebuild());
        }
        Promise.all(promises).then(async () => {
            for (const context of contexts) {
                await context.dispose();
            }
        }).then(() => { return undefined; }).catch(console.error);
    }
}).catch(console.error);