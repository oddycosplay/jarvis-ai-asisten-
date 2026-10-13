# Product Requirements Document (PRD)

**Produk:** JARVIS OS UI & Interactive Dashboard (Web-based Mockup Edition)
**Platform:** Cross-platform Desktop UI Simulation (HTML5, Tailwind CSS, JavaScript)
**Versi Dokumen:** 3.0 (Pembaruan Fitur Interaktif & Modul Sistem)
**Status:** Aktif / Selesai

## 1. Ringkasan Eksekutif (Executive Summary)

**JARVIS OS UI Mockup** adalah representasi antarmuka sistem operasi cerdas berbasis web dengan konsep *glassmorphic design* yang sangat imersif. Versi terbaru ini telah diperkaya dengan fungsionalitas interaktif penuh di seluruh tab utama—mulai dari panel kontrol cepat, widget jam multi-tema, manajemen kalender & jadwal berbasis tanggal, manajemen *clipboard* *split-view*, hingga pusat notifikasi ala Windows OS yang terintegrasi dengan pemantau performa perangkat keras (*CPU, RAM, Disk*).

## 2. Fitur Utama & Ruang Lingkup Fungsional (Core Features)

### 2.1. Navigasi & Transisi Desktop
* **Mode Maskot (Mini Island):** Ikon hantu minimalis di bagian atas layar yang berfungsi sebagai titik akses utama.
* **Expanded Panel:** Panel melayang dinamis dengan ukuran yang disesuaikan secara proporsional untuk menghindari ruang kosong berlebih (*negative space*).
* **Desktop Click-Outside Listener:** Mengeklik area di luar panel utama secara otomatis menutup panel dan mengembalikannya ke bentuk ikon maskot.

### 2.2. Tab Kontrol & Media (*Controls*)
* **Quick Toggles:** Tombol interaktif untuk Wi-Fi, Bluetooth, Audio, Night Light, dan Dark Mode (lengkap dengan perubahan status visual).
* **Media Player Integrasi:** Widget pemutar musik *Lo-fi Beats* dengan tombol *Play/Pause*, *Skip/Previous*, dan *progress bar* yang interaktif.
* **System Utilities:** Pintasan cepat untuk *Screenshot*, *Screen Recording*, *Voice Assistant*, dan *Performance Monitor*.

### 2.3. Tab Produktivitas & Jadwal (*Today*)
* **Dynamic Clock & Theme Switcher:** Jam digital *real-time* yang dapat diklik untuk mengubah tema warna secara instan (Default Dark, Gold Accent, Neon Glass).
* **Pomodoro / Timer Interaktif:** Pengatur waktu melingkar dengan pilihan cepat (1m, 5m, 10m, dll.), tombol *Start/Pause*, *Reset*, dan animasi *progress bar*.
* **Interactive Calendar & Task Binding:** 
  * Navigasi minggu menggunakan tombol panah kiri/kanan.
  * Mengklik tanggal tertentu memungkinkan penambahan tugas baru yang langsung terasosiasi dan tampil secara spesifik untuk tanggal tersebut di bawah kalender.
* **To-Do List Dinamis:** Penghitung tugas selesai (*checkbox* interaktif dengan coret teks).

### 2.4. Pusat Notifikasi & Perangkat (*Stats / Notifications*)
* **Windows OS Notification Center:** Menampilkan daftar notifikasi nyata dari aplikasi (WhatsApp, media sosial, pembaruan sistem).
  * *Interaksi:* Pengguna dapat menghapus notifikasi satu per satu atau membersihkan semuanya sekaligus (*Clear All*).
* **Laptop Performance Monitor:** Widget pemantauan langsung penggunaan sumber daya komputer meliputi:
  * **CPU Usage** (Grafik/Bar dinamis)
  * **Memory / RAM** (Penggunaan aktual vs total kapasitas)
  * **SSD Disk** (Sisa ruang penyimpanan bebas)

### 2.5. Tab Manajemen Klip (*Clipboard*)
* **Split-View Interface:** Panel kiri untuk daftar riwayat (*Pinned*, *Today*, *Yesterday*) dengan ikon jenis data (Teks, Tautan, Perintah, Gambar).
* **Detail & Metadata Panel:** Panel kanan yang menampilkan isi teks lengkap beserta ukuran karakter, jenis, dan waktu disalin.
* **Pencarian & Filter:** Bilah pencarian instan dan tombol filter kategori (*All*, *Text*, *Images*, *Links*).

### 2.6. Pengaturan Lainnya
* **Apps:** Direktori aplikasi tiruan dengan filter kategori (Office, Coding, Media, dll.).
* **Wallpaper:** Galeri preset latar belakang dan generator prompt AI.
* **Power:** Menu kontrol daya sistem lengkap dengan mode **Lock**, **Sleep**, **Restart**, dan **Shut Down**.

## 3. Spesifikasi Teknis & Kualitas Perangkat Lunak

* **Single-File Architecture:** Seluruh struktur tata letak (HTML), gaya visual (Tailwind CSS/CSS Custom), dan logika interaktif (JavaScript) digabungkan ke dalam satu file tunggal untuk kemudahan pengujian dan portabilitas.
* **Error Handling:** Pemeriksaan elemen DOM (*null-safety checks*) diterapkan di seluruh fungsi inisialisasi skrip guna mencegah galat konsol saat pemuatan halaman (*startup*).