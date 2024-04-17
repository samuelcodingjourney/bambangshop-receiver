# BambangShop Receiver App
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
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman 

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
1. If mutex is used instead of RwLock in the case to synchronise the use of Vec of Notifications, then the problem is because Mutex provide exclusive access for both reading and writing. This means that multiple subscribers would have to wait for that exclusive access when they only want to read the data while RwLock could allow multiple reader to access the data or the notification simultaneously while that publisher is being the only writer at a time. So RwLock is more efficiently to be used than Mutex. This would prevent blocking when multiple subscribers wants to read the notification when using RwLock.
2. Rust does not allow the mutation of the content of a static variable via static function like in Java because of several reasons. The definition of the static keyword in Rust is that the variable will live in the entire duration of the program execution. The first reason is thread safety. If the static variable is mutable then there could be problems during multi-threaded that multiple threads try to mutate the variable simultaneously. This is a data racing problem. Rust also guarantees initialization of static variables before accessed so Rust does now allow mutability that could make the initialization process more complex and could lead to more unsafe program behavior when run. Another reason is that Rust protects its memory really tightly as to prevent memory loss or corruption or even any improper behavior in the result of mutable static variables that could be modified from anywhere in the project. That is why in Rust mutex and RwLock could be used for safe concurrency to tackle this problem. 
#### Reflection Subscriber-2
1. Yes, in lib.rs this file is used for setting up the static configurations and handles application-wide configurations. So there is setting up dependencies and imports such as using lazy_static for lazy evaluation of the static variables. Dotenv for loading the environment variables from .env file that is used in the tutorial to create the instantiation of three receiver. Rocket, reqwest, and getset all would be used later on as well. The configuration handling part is that the AppConfig struct has fields for instance_root_url, publisher_root_Url, and instance_name. So basically the generate method loads the environment variables from the .env file that was created and filled out accordingly to dotenv(). Figment would then be used to merge default configurations with those environment-specific configurations and that a configuration management crate would be used that is provided by Rocket framework that is used in this project. Error handling is also applied that contains a response in the form of JSON with fields of status_code and the message. Compose_error_response here is used to create those error responses and providing the necessary status codes and messages. 
2. When talking about more than one publisher or main system, it would definitely take more computational resource to run depending on if the main systems are to be run in the same computer or different hardware entirely meaning that in the scope of this tutorial, multiple ports would be used for both main systems and the subscribers to the main systems. Adding more main systems could lead to more problems if for example they are handling a shared resources or state after all this is Rust that we are talking about that was used to build this observer pattern. If not properly managed or monitored when for example adding new subscribers for certain main systems then there could be problems there to be exploited.
3. Yes it has been helping me mostly on doing this tutorial work because I can test the endpoints and especially for observer pattern, there are different systems to be tested which are the main systems and the subscribers part. I was managed to debug problems like missing or typo syntatic mistakes when using Postman. 