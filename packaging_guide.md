# Panduan Mengubah JARVIS OS Mockup Menjadi Aplikasi Windows (.exe)

Dokumen ini berisi panduan lengkap langkah demi langkah untuk mengemas (*packaging*) aplikasi web interaktif **JARVIS OS** Anda menjadi aplikasi desktop Windows asli berformat `.exe` menggunakan **Electron** dan **electron-builder**.

---

## 1. Prasyarat Sistem
Sebelum memulai, pastikan komputer Anda telah terinstal:
* **Node.js** (Versi 18.x atau terbaru - unduh dari [nodejs.org](https://nodejs.org/))
* **Git** (Opsional, untuk manajemen repositori)

---

## 2. Struktur Direktori Proyek
Buat satu folder baru di komputer Anda (misalnya `jarvis-desktop`), lalu simpan file utama HTML Anda dengan nama **`index.html`** di dalam folder tersebut.

Struktur folder Anda harus seperti ini:
```text
jarvis-desktop/
├── index.html       # File utama JARVIS OS yang telah kita buat
├── main.js          # File skrip utama Electron (dibuat di langkah 3)
└── package.json     # Konfigurasi proyek & dependensi (dibuat di langkah 4)
```

---

## 3. Membuat File Konfigurasi `package.json`
Buat file bernama `package.json` di root folder Anda dan isi dengan konfigurasi berikut:

```json
{
  "name": "jarvis-os-desktop",
  "productName": "JARVIS OS",
  "version": "3.0.0",
  "description": "The Ultimate Productivity Tool & Windows OS Mockup",
  "main": "main.js",
  "scripts": {
    "start": "electron .",
    "build": "electron-builder --win --x64"
  },
  "author": "Mainak & Team",
  "license": "MIT",
  "devDependencies": {
    "electron": "^28.2.0",
    "electron-builder": "^24.9.1"
  },
  "build": {
    "appId": "com.jarvis.os.desktop",
    "win": {
      "target": "nsis",
      "icon": "assets/icon.ico"
    },
    "directories": {
      "output": "dist"
    }
  }
}
```

---

## 4. Membuat File Penghubung `main.js` (Electron Main Process)
Buat file bernama `main.js` di root folder yang sama untuk menjalankan jendela aplikasi desktop:

```javascript
const { app, BrowserWindow } = require('electron');
const path = require('path');

function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    },
    autoHideMenuBar: true, // Menyembunyikan menu bar bawaan Windows
    title: "JARVIS OS"
  });

  // Memuat file HTML lokal aplikasi JARVIS
  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.stop();
});
```

---

## 5. Proses Build Menjadi File `.exe`

1. Buka **Terminal** atau **Command Prompt (CMD)** pada folder `jarvis-desktop` Anda.
2. Instal pustaka Electron dan Electron Builder dengan menjalankan perintah:
   ```bash
   npm install
   ```
3. Setelah instalasi selesai, jalankan perintah *build* untuk mengompilasi aplikasi menjadi file instalasi `.exe`:
   ```bash
   npm run build
   ```
4. Tunggu beberapa saat hingga proses selesai. Masuk ke folder **`dist/`** yang baru dibuat di dalam direktori Anda. Di dalamnya, Anda akan menemukan file `.exe` siap pakai (misalnya `JARVIS OS Setup 3.0.0.exe`) yang dapat diinstal atau didistribusikan ke perangkat Windows lainnya!