const fs = require('fs')
const path = require('path')

const cwd = process.cwd()

const INPUT_FOLDER = path.join(cwd, 'assets')
const OUTPUT_FOLDER = path.join(cwd, 'bevy_interface', 'assets')

function forEachFileInTree(rootPath, fn) {
  fs.readdirSync(rootPath).forEach(function (file) {
    const filepath = path.join(rootPath, file)
    const stat = fs.statSync(filepath)
    if (stat.isDirectory()) {
      forEachFileInTree(filepath, fn)
    } else {
      fn(filepath)
    }
  })
}

function deleteFolderIfExists(dirPath) {
  if (fs.existsSync(dirPath)) {
    fs.rmSync(dirPath, { recursive: true })
  }
}

function createFolderIfDoesntExist(dirPath) {
  if (!fs.existsSync(dirPath)) {
    fs.mkdirSync(dirPath, { recursive: true })
  }
}

function removeStringPrefix(string, prefix) {
  return string.replace(prefix, '')
}

deleteFolderIfExists(OUTPUT_FOLDER)
createFolderIfDoesntExist(OUTPUT_FOLDER)

forEachFileInTree(INPUT_FOLDER, (filePath) => {
  if (!filePath.startsWith(OUTPUT_FOLDER)) {
    const inputFilePathRelativeToInputFolder = removeStringPrefix(
      filePath,
      INPUT_FOLDER
    )

    let outputFilePath = path.join(
      OUTPUT_FOLDER,
      inputFilePathRelativeToInputFolder
    )
    // Create any new subdirectories needed to write to the output filepath
    createFolderIfDoesntExist(path.dirname(outputFilePath))

    const inputFileContents = fs.readFileSync(filePath)

    fs.writeFileSync(outputFilePath, inputFileContents)
  }
})
