use tokio_postgres::{Error, NoTls, Transaction};

/// Funktion zur Herstellung einer Verbindung zur PostgreSQL-Datenbank
/// Rückgabe: Client und Verbindung (asynchron)
async fn create_connection() -> Result<tokio_postgres::Client, Error> {
    // Verbindungskonfigurationszeichenfolge (anpassen für Ihre Umgebung)
    let connection_str = "host=localhost user=postgres password=postgres dbname=postgres";

    // Verbindung herstellen
    let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;

    // Verbindung in einem separaten Task ausführen
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Datenbankverbindungsfehler: {}", e);
        }
    });

    Ok(client)
}

/// Funktion zum Demonstrieren von Transaktionshandling
/// Beispiel für Insert, Update, Select und Delete in einer Transaktion
async fn handle_transaction(client: &mut tokio_postgres::Client) -> Result<(), Error> {
    // Transaktion starten
    let transaction: Transaction<'_> = client.transaction().await?;
    println!("Transaktion gestartet.");


    // Daten einfügen
    transaction
        .execute(
            "INSERT INTO users (name, age) VALUES ($1, $2)",
            &[&"Bob", &(25 as i32)],
        )
        .await?;

    println!("Daten eingefügt: Name = Bob, Alter = 25.");

    // Daten aktualisieren
    transaction
        .execute(
            "UPDATE users SET age = $1 WHERE name = $2",
            &[&(30 as i32), &"Bob"],
        )
        .await?;
    println!("Daten aktualisiert: Name = Bob, Alter = 30.");

    // Daten abfragen
    let rows = transaction
        .query("SELECT id, name, age FROM users WHERE name = $1", &[&"Bob"])
        .await?;
    for row in rows {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("Abgefragte Daten - ID: {}, Name: {}, Alter: {}", id, name, age);
    }

    // Daten löschen
    transaction
        .execute("DELETE FROM users WHERE name = $1", &[&"Bob"])
        .await?;
    println!("Daten gelöscht: Name = Bob.");

    // Transaktion festschreiben (Commit)
    transaction.commit().await?;
    println!("Transaktion erfolgreich festgeschrieben.");

    // Alternativ: Wenn ein Fehler auftritt, wird die Transaktion automatisch zurückgerollt,
    // falls `commit()` nicht aufgerufen wird.

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Verbindung erstellen
    let mut client = create_connection().await?;

    // Tabelle erstellen (einmalig, für Demonstrationszwecke)
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                age INT NOT NULL
            )",
            &[],
        )
        .await?;
    println!("Tabelle 'users' erstellt.");

    // Transaktionshandling durchführen
    handle_transaction(&mut client).await?;

    Ok(())
}
