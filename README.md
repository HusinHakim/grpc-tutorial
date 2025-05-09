# gRPC Tutorial Reflection

## 1. Perbedaan antara Unary, Server Streaming, dan Bi-directional Streaming RPC

- **Unary RPC**: Klien mengirim satu permintaan dan server mengembalikan satu respons. Cocok untuk operasi tradisional seperti mengambil atau memperbarui data tunggal (contoh: `process_payment` dalam aplikasi ini).

- **Server Streaming RPC**: Klien mengirim satu permintaan dan server mengembalikan stream respons. Ideal untuk kasus di mana klien perlu menerima banyak data atau pembaruan berkelanjutan (contoh: `get_transaction_history` dalam aplikasi ini).

- **Bi-directional Streaming RPC**: Baik klien maupun server dapat mengirim stream pesan secara bersamaan. Sangat cocok untuk komunikasi real-time seperti aplikasi chat, game online, atau kolaborasi real-time (contoh: `chat` dalam aplikasi ini).

## 2. Pertimbangan Keamanan dalam Implementasi gRPC di Rust

- **Autentikasi**: Implementasi gRPC harus mendukung mekanisme autentikasi seperti OAuth, JWT, atau SSL/TLS client certificates.

- **Otorisasi**: Setelah pengguna diautentikasi, layanan harus memeriksa izin mereka untuk mengakses sumber daya tertentu.

- **Enkripsi Data**: Komunikasi gRPC harus diamankan dengan TLS untuk melindungi data dalam transit.

- **Validasi Input**: Validasi semua input pengguna untuk mencegah serangan injeksi.

- **Rate Limiting**: Menerapkan pembatasan rate untuk mencegah serangan DoS.

## 3. Tantangan dalam Menangani Bidirectional Streaming di Rust gRPC

- **Pengelolaan Kesalahan**: Menangani kesalahan di kedua sisi stream secara independen.

- **Konkurensi**: Mengelola konkurensi dengan benar menggunakan Tokio runtime.

- **Backpressure**: Menangani backpressure saat salah satu sisi memproses pesan lebih lambat.

- **Koneksi Terputus**: Mendeteksi dan menangani klien yang terputus secara tiba-tiba.

- **State Management**: Menjaga status koneksi dan mengelola sumber daya dengan benar.

## 4. Kelebihan dan Kekurangan ReceiverStream

**Kelebihan**:
- Mengintegrasikan channel Tokio dengan gRPC secara mulus
- Memungkinkan komunikasi asinkron antar task
- Menyediakan backpressure built-in
- Menyederhanakan konversi dari receiver Tokio ke gRPC stream

**Kekurangan**:
- Overhead tambahan karena wrapping
- Memerlukan pemahaman tentang model konkurensi Tokio
- Bisa rumit untuk men-debug isu dengan channel
- Potensi deadlock jika tidak dikelola dengan benar

## 5. Struktur Kode untuk Reusabilitas dan Modularitas

- **Pemisahan Layanan**: Pisahkan implementasi layanan dari pengendali bisnis.

- **Abstraksi Database**: Gunakan trait untuk abstraksi akses data.

- **Error Handling**: Buat tipe error kustom yang dapat dikonversi ke status gRPC.

- **Konfigurasi**: Pisahkan konfigurasi aplikasi dari kode bisnis.

- **Testing**: Desain untuk kemudahan pengujian dengan mock dan dependency injection.

## 6. Langkah Tambahan untuk Logic Pemrosesan Pembayaran Kompleks

- **Validasi Pembayaran**: Validasi input dan batasan bisnis.

- **Integrasi dengan Gateway Pembayaran**: Hubungkan dengan penyedia pembayaran pihak ketiga.

- **Logging dan Audit**: Catat semua transaksi untuk audit.

- **Transaksi Database**: Gunakan transaksi untuk menjaga konsistensi data.

- **Notifikasi**: Kirim notifikasi ke sistem lain tentang status pembayaran.

## 7. Dampak gRPC pada Arsitektur Sistem Terdistribusi

- **Kontrak Service yang Tepat**: Definisi protofile memberikan kontrak yang jelas antar layanan.

- **Efisiensi Komunikasi**: HTTP/2 mendukung multiplexing dan kompresi header.

- **Bahasa Agnostik**: Kemampuan untuk menghasilkan client dan server di berbagai bahasa.

- **Integrasi Mikro-servis**: Mempermudah komunikasi antar layanan mikro.

- **Kompleksitas Sistem**: Sistem bisa menjadi lebih kompleks karena diperlukan pengelolaan protokol baru.

## 8. HTTP/2 vs HTTP/1.1 untuk REST API

**Kelebihan HTTP/2**:
- Multiplexing permintaan dalam satu koneksi
- Kompresi header
- Server push
- Prioritisasi permintaan
- Komunikasi biner yang lebih efisien

**Kekurangan HTTP/2**:
- Kompleksitas implementasi yang lebih tinggi
- Debugging yang lebih sulit karena format biner
- Masih membutuhkan TLS di sebagian besar implementasi
- Memerlukan proxy dan infrastruktur yang mendukung

## 9. gRPC vs REST untuk Komunikasi Real-time

**gRPC**:
- Mendukung streaming bidreksional untuk komunikasi real-time
- Latency lebih rendah karena HTTP/2
- Efisiensi bandwidth lebih baik dengan Protocol Buffers

**REST**:
- Perlu polling atau WebSocket untuk fitur real-time
- Model permintaan-respons tradisional kurang ideal untuk pembaruan real-time
- Overhead lebih tinggi untuk komunikasi berkelanjutan
- Lebih sederhana untuk implementasi sederhana

## 10. Pendekatan Schema-based vs Schema-less

**Protocol Buffers (gRPC)**:
- Kontrak yang terdefinisi dengan jelas
- Validasi tipe data built-in
- Ukuran payload yang lebih kecil
- Kinerja serialisasi/deserialisasi yang lebih baik
- Evolusi skema yang terkontrol

**JSON (REST)**:
- Fleksibilitas yang lebih tinggi
- Mudah untuk dibaca dan debug manusia
- Tidak memerlukan alat khusus untuk inspeksi
- Lebih mudah untuk diubah dan dikembangkan
- Support lebih luas di browser dan alat 