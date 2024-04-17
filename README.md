# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection 1

1. Tergantung kompleksitas pada kasus yang dialami. Pada kasus semua subscriber dianggap hanya sebagai listener untuk perubahan, 1 buah struct cukup. Pada kasus dimana terdapat jenis yang berbeda, maka diperlukan trait yang berbeda untuk setiap jenis.
2. Menggunakan Vec tidak cukup, karena tidak ada yang menjamin keunikan dari id dan url jika seperti itu sehingga DashMap diperlukan untuk kasus ini
3. Kedua kasus bisa diterapkan. Singleton dapat dibuat secara thread-safe dengan menggunakan RWLock atau Mutex.

#### Reflection 2

1. Karena dengan memisahkan Service dan Repository, masing-masing class tersebut hanya memiliki 1 tanggung jawab. Service hanya mengurus business logic dan Repository mengurus pengambilan data dari database. Ketika dibutuhkan modifikasi maka akan mudah dilakukan karena coupling antara Service dan Repository tidak seerat itu.
2. Jika menggunakan model, maka ketiga model akan memiliki coupling yang sangat erat sehingga menyulitkan modifikasi tanpa memengaruhi bagian lain.
3. Postman membantu saya untuk menembak ke endpoint yang saya perlukan. Alat ini memudahkan saya untuk mengakses endpoint yang methodnya selain GET request.

#### Reflection 3

1. Variasi yang digunakan pada kasus ini adalah model Push. Hal ini ditunjukkan pada cara yang digunakan untuk nofity pengguna, ketika create / delete, dibuat POST request ke endpoint yang sesuai untuk mengubah informasi yang diperlukan.
2. Pada model pull subscriber yang melakukan update sendiri. Kasus ini lebih ramah untuk server karena endpoint yang bersangkuta tak selalu di-hit.
3. Mengirim notifikasi tanpa multi-threading akan lebih lambat daripada dengan multi-threading. Hal ini disebabkan karena proses update setiap pengguna akan bersifat blocking.