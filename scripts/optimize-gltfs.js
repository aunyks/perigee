import { DenoIO } from 'https://esm.sh/@gltf-transform/core'
import { KHRONOS_EXTENSIONS } from 'https://esm.sh/@gltf-transform/extensions'
import {
  resample,
  prune,
  dedup,
  textureResize,
} from 'https://esm.sh/@gltf-transform/functions'
import * as path from 'https://deno.land/std/path/mod.ts'

async function getNestedGltfs(searchPath) {
  let names = []

  for await (const dirEntry of Deno.readDir(searchPath)) {
    const entryPath = `${searchPath}/${dirEntry.name}`
    if (dirEntry.isDirectory) {
      names.push(await getNestedGltfs(entryPath))
    } else {
      if (entryPath.endsWith('.glb')) {
        names.push(entryPath)
      }
    }
  }

  return names.flat(1)
}

const io = new DenoIO().registerExtensions(KHRONOS_EXTENSIONS)

const gltfDirectory = path.joinGlobs([Deno.cwd(), 'assets', 'gltf'])
for (const gltfPath of await getNestedGltfs(gltfDirectory)) {
  const glbBytes = await Deno.readFile(gltfPath)
  const glbDocument = await io.readBinary(glbBytes)

  await glbDocument.transform(
    dedup(),
    resample(),
    prune(),
    textureResize({ size: [1024, 1024] })
  )

  const newGlbBytes = await io.writeBinary()
  await Deno.writeFile(gltfPath, newGlbBytes)
}
