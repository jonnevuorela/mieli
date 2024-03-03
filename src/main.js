import { appWindow } from '@tauri-apps/api/window'

document.getElementById('titlebar-minimize').addEventListener('click', () => appWindow.minimize())
document.getElementById('titlebar-maximize').addEventListener('click', () => appWindow.toggleMaximize())
document.getElementById('titlebar-close').addEventListener('click', () => appWindow.close())

document.getElementById('addNew').addEventListener('click', () => {window.__TAURI__.invoke('addNewDiv');});