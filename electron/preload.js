// Preload script for Electron
// Exposes safe APIs to the React frontend

const { contextBridge, ipcRenderer } = require('electron')

// Example: expose an API for window controls if needed
contextBridge.exposeInMainWorld('electronAPI', {
  // Add IPC channels here
})
