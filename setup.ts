#!/usr/bin/env -S deno run --allow-write --allow-read --allow-run=bash,git,cargo --allow-net=docs.rs:443 --allow-env --allow-sys --no-lock

// NOTE: Pin the versions of the packages because the script runs without a lock file
import { dirname, SEPARATOR } from "jsr:@std/path@0.224.0"
import { exists } from "jsr:@std/fs@0.224.0/exists"
import * as zx from "npm:zx@8.3.2"
import { parseArgs } from "jsr:@std/cli@1.0.13"

// Must work with GitHub repos
// Must work with Keybase repos

export const { target } = parseArgs(Deno.args, {
    string: "target",
    alias: {
        target: "t",
    },
})

if (!target) throw new Error("--target option is required")

export const $ = zx.$({ cwd: target })

export const getSource = async (localPath: string, repoUrl: string) => {
    if (await exists(localPath)) {
        return localPath
    } else {
        const tempDir = await Deno.makeTempDir({
            prefix: "git-clone-",
        })
        await $`git clone --depth 1 ${repoUrl} ${tempDir}`
        return tempDir
    }
}

export const copy = (from: string, to: string) => async (relativePath: string) => {
    const source = from + SEPARATOR + relativePath
    const target = to + SEPARATOR + relativePath
    const targetParent = dirname(target)
    await Deno.mkdir(targetParent, { recursive: true })
    await Deno.copyFile(source, target)
}

// launch multiple promises concurrently
const sourcePromise = getSource(Deno.env.get("HOME") + "/workspace/url-macro", "https://github.com/DenisGorbachev/url-macro")
const remoteUrlPromise = $`git remote get-url origin`

export const source = await sourcePromise
export const cp = copy(source, target)
export const cpAll = (paths: string[]) => paths.map(cp)

const promises = cpAll([
    "mise.toml",
    "lefthook.yml",
    "rustfmt.toml",
    "commitlint.config.mjs",
    "deno.json",
    "README.ts",
    ".github/workflows/ci.yml",
    "LICENSE-APACHE",
    "LICENSE-MIT",
])

export const remoteUrl = await remoteUrlPromise
await Promise.all(promises)

const miseTrustPromise = $`mise trust ${target}/mise.toml`
await miseTrustPromise

// Test that everything works
const lefthookPromise = $`lefthook run -f pre-commit`
await lefthookPromise
