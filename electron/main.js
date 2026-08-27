import { app, BrowserWindow } from 'electron'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'
import fs from 'fs'

fs.writeFileSync('electron.log', 'Electron starting...\n')
process.on('uncaughtException', (err) => {
  fs.appendFileSync('electron.log', 'Error: ' + err.message + '\n' + err.stack + '\n')
})

const __dirname = dirname(fileURLToPath(import.meta.url))

process.env.DIST_ELECTRON = join(__dirname, '../dist-electron')
process.env.DIST = join(__dirname, '../dist')
process.env.PUBLIC = process.env.VITE_DEV_SERVER_URL
  ? join(__dirname, '../public')
  : process.env.DIST

let win = null

function createWindow() {
  fs.appendFileSync('electron.log', 'createWindow called...\n')
  win = new BrowserWindow({
    width: 1200,
    height: 800,
    title: 'Granco Planner',
    icon: join(process.env.PUBLIC, 'favicon.svg'),
    webPreferences: {
      preload: join(__dirname, 'preload.js'),
      nodeIntegration: false,
      contextIsolation: true,
    },
  })

  win.once('ready-to-show', () => {
    fs.appendFileSync('electron.log', 'ready-to-show fired! Forcing show.\n')
    win.show()
    win.focus()
  })

  // Set minimum size for better UX
  win.setMinimumSize(800, 600)
  win.center()
  win.webContents.openDevTools({ mode: 'detach' })

  fs.appendFileSync('electron.log', 'Window created, loading URL...\n')

  if (process.env.VITE_DEV_SERVER_URL) {
    fs.appendFileSync('electron.log', 'Loading dev server: ' + process.env.VITE_DEV_SERVER_URL + '\n')
    win.loadURL(process.env.VITE_DEV_SERVER_URL).catch(e => fs.appendFileSync('electron.log', 'Load err: ' + e + '\n'))
  } else {
    fs.appendFileSync('electron.log', 'Loading local file: ' + join(process.env.DIST, 'index.html') + '\n')
    win.loadFile(join(process.env.DIST, 'index.html')).catch(e => fs.appendFileSync('electron.log', 'Load err: ' + e + '\n'))
  }
}

app.commandLine.appendSwitch('disable-http-cache')
app.commandLine.appendSwitch('disable-gpu')
app.commandLine.appendSwitch('disable-gpu-shader-disk-cache')

app.whenReady().then(() => {
  fs.appendFileSync('electron.log', 'app.whenReady resolved!\n')
  createWindow()
}).catch(err => {
  fs.appendFileSync('electron.log', 'app.whenReady error: ' + err.message + '\n')
})

app.on('window-all-closed', () => {
  win = null
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow()
  }
})
