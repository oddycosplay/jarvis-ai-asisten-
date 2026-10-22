const { app, BrowserWindow } = require('electron');
const path = require('path');
const fs = require('fs');

app.whenReady().then(async () => {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    show: false,
    frame: false,
    backgroundColor: '#0a0a0a',
    webPreferences: {
      contextIsolation: false,
      nodeIntegration: true
    }
  });

  const screenshotsDir = path.join(__dirname, '..', 'assets', 'screenshots');
  if (!fs.existsSync(screenshotsDir)) {
    fs.mkdirSync(screenshotsDir, { recursive: true });
  }

  async function resetAndLoad() {
    await win.loadFile(path.join(__dirname, '..', 'jarvis_ui_mockup.html'));
    await new Promise(r => setTimeout(r, 1200));
  }

  // 1. Collapsed Mini Mascot Island
  await resetAndLoad();
  let img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '01_dynamic_island_collapsed.png'), img.toPNG());
  console.log('Saved 01_dynamic_island_collapsed.png');

  // 2. Contextual Island (Media Player)
  await resetAndLoad();
  await win.webContents.executeJavaScript(`
    const cp = document.getElementById('context-popup');
    if (cp) {
      cp.classList.remove('hidden', 'animate-slide-down-panel', 'animate-slide-up-panel');
      cp.style.animation = 'none';
      cp.style.transform = 'translateX(-50%) translateY(0)';
      cp.style.opacity = '1';
    }
  `);
  await new Promise(r => setTimeout(r, 400));
  img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '02_contextual_island_media.png'), img.toPNG());
  console.log('Saved 02_contextual_island_media.png');

  // 3. Expanded Panel - Controls Tab
  await resetAndLoad();
  await win.webContents.executeJavaScript(`
    openPanelTab('controls');
  `);
  await new Promise(r => setTimeout(r, 400));
  img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '03_expanded_panel_controls.png'), img.toPNG());
  console.log('Saved 03_expanded_panel_controls.png');

  // 4. Expanded Panel - Hardware Telemetry & Notifications Tab
  await resetAndLoad();
  await win.webContents.executeJavaScript(`
    openPanelTab('stats');
  `);
  await new Promise(r => setTimeout(r, 400));
  img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '04_expanded_panel_hardware_telemetry.png'), img.toPNG());
  console.log('Saved 04_expanded_panel_hardware_telemetry.png');

  // 5. Expanded Panel - Apps Launcher Tab
  await resetAndLoad();
  await win.webContents.executeJavaScript(`
    openPanelTab('apps');
  `);
  await new Promise(r => setTimeout(r, 400));
  img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '05_expanded_panel_apps.png'), img.toPNG());
  console.log('Saved 05_expanded_panel_apps.png');

  // 6. JARVIS Multi-Model AI Assistant Modal
  await resetAndLoad();
  await win.webContents.executeJavaScript(`
    const modal = document.getElementById('ai-assistant-modal');
    modal.classList.remove('hidden', 'animate-pop-in');
    modal.style.animation = 'none';
    modal.style.transform = 'translate(-50%, -50%)';
    modal.style.opacity = '1';
  `);
  await new Promise(r => setTimeout(r, 400));
  img = await win.webContents.capturePage();
  fs.writeFileSync(path.join(screenshotsDir, '06_jarvis_ai_assistant_modal.png'), img.toPNG());
  console.log('Saved 06_jarvis_ai_assistant_modal.png');

  console.log('ALL SCREENSHOTS CAPTURED PERFECTLY!');
  app.quit();
});
