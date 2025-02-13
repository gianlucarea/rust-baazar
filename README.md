# Rust Baazar 🦀
Welcome to my **Rust Bazaar** repository! This space is dedicated entirely to exploring and mastering **Rust**. Over time, I've created **4 projects** in Rust, each aiming to enhance my understanding of the language and its rich ecosystem.

## **Repository Goal**

The primary goal of this repository is to serve as a personal learning log, showcasing my progress and development in Rust. Each project is a small step in my journey, expanding my horizons with every line of code. With each new project, I am able to explore different Rust features, libraries, and tools that make Rust a powerful language for modern software development.

## **Projects 🦀**

### [Grrs 🧨](./grrs)
This Rust CLI project called Grrs covers ownership, borrowing, `fs::File`, and `io::BufRead`. It implements basic `cat`, `grep`, `cp`, `mv`, and `rm` commands.

### [Runestone 🗿](./runestone)
This project, Runestone, is a minimal Rust web server built with Axum and Tokio for asynchronous programming with CRUD operations. It integrates PostgreSQL. 
| **Method** | **Route**                           | **Description**                                   |
|------------|-------------------------------------|---------------------------------------------------|
| GET        | `/`                                 | Test route: `"Hello Aldino!"`                     |
| GET        | `/runestones`                       | Retrieve all runestones                          |
| POST       | `/runestones`                       | Create a new runestone                            |
| GET        | `/runestones/{runestone_id}`        | Get a specific runestone by `runestone_id`        |
| PATCH      | `/runestones/{runestone_id}`        | Update a specific runestone by `runestone_id`     |
| DELETE     | `/runestones/{runestone_id}`        | Delete a specific runestone by `runestone_id`     |


### [Archimedes MQ 🛀](./archimedes-mq)
Archimedes MQ is an async Rust IoT data logger using Tokio and MQTT, simulating sensors and storing real-time readings in JSON.

### [Byzantine Vault 🏛️](./byzantine-vault/)
ByzantineVault is a secure file storage system. It combines encryption (AES-GCM), JWT authentication, and PostgreSQL-backed secure access control
| **Method** | **Route**                                             | **Description**                                  |
|------------|-------------------------------------------------------|--------------------------------------------------|
| GET        | `/`                                                   | Test route: `"Hello Aldino!"`                    |
| POST       | `/register`                                           | Register a new user                             |
| POST       | `/login`                                              | Login and generate JWT                          |
| POST       | `/auth/{owner_id}/files/`                             | Upload a file                                   |
| GET        | `/auth/{owner_id}/files/{file_name}/`                 | Download a file                                 |
| DELETE     | `/auth/{owner_id}/files/{file_name}/`                 | Delete a file                                   |
| GET        | `/auth/{owner_id}/files/{file_name}/version/{version}/`| Download file by version                           |
| DELETE     | `/auth/{owner_id}/files/{file_name}/version/{version}/`| Delete file by version                             |

### [Lionheart Armory 🦁](./byzantine-vault/)
Lionhearth Armory is a secure armory api. JWT authentication, and PostgreSQL-backed secure access control and Redis Cache.
---

## **Project Updates**

As I complete each project, the README will be updated to reflect the latest work. You’ll find detailed descriptions and any lessons learned from the projects in the repository.

---

Thank you for visiting my Rust Bazaar repository! Feel free to browse, learn, and share ideas. I hope this collection inspires others to dive into Rust and explore its potential.

Happy coding with Rust! 🚀
