# testownik-ng

Testownik to aplikacja do nauki i zapamiętywania za pomocą fiszek. Nasz projekt
umożliwia zarządzanie pakietami fiszek oraz pobieranie z centralnego
repozytorium.

- [Testownik](https://github.com/TestownikiPWR/testownik-electron)

## Run Locally

Clone the project

```bash
git clone https://link-to-project
```

Go to the project directory

```bash
cd testownik-ng/server
```

Install dependencies and setup

```bash
./setup.sh
```

Start the server

```bash
./run.sh
```

## API Reference

#### Get metadata of all packages

```http
GET /get_metadata_all
```

| Parameter | Type   | Description                                                  |
| :-------- | :----- | :----------------------------------------------------------- |
| `None`    | `None` | Returns a json list of all aviable packages and their hashes |

#### Get package metadata from hash

```http
GET /get_package_metadata/?hash=
```

| Parameter | Type     | Description                            |
| :-------- | :------- | :------------------------------------- |
| `hash`    | `string` | **Required**. Hash/ID of item to fetch |

## Authors

- [@Michał Bernacki](https://www.github.com/bberni)

- [@Wiktor Rojecki](https://www.github.com/R0JSON)

- [@Mateusz Jakoniuk](https://www.github.com/JakeQusha)
- [@Karol Gębski](https://www.github.com/KGebski0036)

[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/license/gpl-3-0)
