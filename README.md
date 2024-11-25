# PostgreSQL mit `tokio-postgres` in Rust

Dieses Projekt zeigt ein einfaches Beispiel für die Nutzung von PostgreSQL mit der Rust-Bibliothek `tokio-postgres`. Es enthält eine Datenbankverbindung, eine Transaktion zum Einfügen von Daten und grundlegende Abfragen.

## Voraussetzungen

### 1. Installierte Software
Um dieses Projekt auszuführen, benötigen Sie:
- **Rust**: Installieren Sie die neueste Version von Rust mit [rustup](https://rustup.rs/).
- **Docker**: Installieren Sie Docker und Docker Compose, um die PostgreSQL-Datenbank aufzusetzen.

## Projektstruktur

### Dateien
- `src/main.rs`: Der Rust-Code mit der Verbindung zur PostgreSQL-Datenbank und einem Beispiel für Transaktionen.
- `Cargo.toml`: Die Projektabhängigkeiten.

---

## Installation

### 1. Rust-Projekt einrichten
Klonen Sie dieses Repository und wechseln Sie in das Verzeichnis:
```bash
git clone https://github.com/FelixWieland/dhbw-ti23-rust-postgres
cd dhbw-ti23-rust-postgres
```

Installieren Sie die Abhängigkeiten:
```bash
cargo build
```

Passen Sie `user`, `password` und `dbname` entsprechend an, wenn Sie andere Werte in `docker-compose.yml` verwendet haben.

---

## Beispiel ausführen

1. Starten Sie die PostgreSQL-Datenbank mit Docker Compose (falls noch nicht geschehen):
   ```bash
   docker-compose up -d
   ```

2. Führen Sie das Programm aus:
   ```bash
   cargo run
   ```

Wenn alles korrekt eingerichtet ist, sollten Sie in der Konsole eine Bestätigung über den erfolgreichen Datenbankeintrag sehen:
```
Tabelle 'users' erstellt.
Transaktion gestartet.
Daten eingefügt: Name = Bob, Alter = 25.
Daten aktualisiert: Name = Bob, Alter = 30.
Abgefragte Daten - ID: 2, Name: Bob, Alter: 30
Daten gelöscht: Name = Bob.
Transaktion erfolgreich festgeschrieben.
```

---

## Erklärung des Codes

### 1. Datenbankverbindung
Die Verbindung zur Datenbank wird mit `tokio-postgres` hergestellt:
```rust
let (client, connection) = tokio_postgres::connect(
    "host=localhost user=postgres password=postgres dbname=postgres",
    NoTls,
)
.await?;
```

- `host`: Die IP oder der Hostname der Datenbank (`localhost` für Docker Compose).
- `user`: Der Benutzername (`postgres`).
- `password`: Das Passwort (`postgres`).
- `dbname`: Der Name der Datenbank (`postgres`).

Die Verbindung läuft in einem separaten Task, um die Asynchronität zu gewährleisten.

### 2. Transaktion
Daten werden in einer Transaktion eingefügt:
```rust
let transaction = client.transaction().await?;
transaction
    .execute(
        "INSERT INTO users (name, age) VALUES ($1, $2)",
        &[&"Bob", &25],
    )
    .await?;
transaction.commit().await?;
```

- `$1` und `$2` sind Platzhalter für Parameter.
- Die Parameter werden als Referenzen übergeben, z. B. `&"Bob"` und `&25`.

### 3. Abfragen und Verarbeitung
Ein Beispiel für eine Abfrage und den Abruf von Daten:
```rust
let row = client
    .query_one("SELECT name FROM users WHERE id = $1", &[&1])
    .await?;
let name: String = row.get(0);
println!("User name: {}", name);
```

---

## Nützliche Befehle

### Docker
- **Container stoppen**:
  ```bash
  docker-compose down
  ```

- **Logs anzeigen**:
  ```bash
  docker-compose logs
  ```

### PostgreSQL-Datenbank
Um sich mit der Datenbank zu verbinden:
```bash
docker exec -it rust_postgres_example_db psql -U postgres -d postgres
```

---

## Fehlerbehebung

1. **Verbindungsfehler**:
   - Stellen Sie sicher, dass der Docker-Container läuft:
     ```bash
     docker ps
     ```
   - Prüfen Sie die Verbindung zu `localhost:5432`.

2. **Typfehler bei Parametern**:
   - Vergewissern Sie sich, dass die Typen der Rust-Parameter mit den Datenbankspalten übereinstimmen (z. B. `i32` für `INTEGER`).

3. **Keine Berechtigungen**:
   - Prüfen Sie Benutzername und Passwort in `docker-compose.yml` und im Rust-Code.

---

## Nächste Schritte

- Erweiterung des Codes, um mehrere Tabellen und komplexere Queries zu handhaben.
- Einsatz von Migrations-Tools wie `diesel` oder `sqlx` für Schemaänderungen.
- Automatische Tests mit einer isolierten Testdatenbank.
