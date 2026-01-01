use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs;
use std::path::Path;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct CelestialObject {
    pub id: i64,
    pub name: String,
    pub distance_ly: f64,
    pub object_type: String,
    pub description: String,
}

pub async fn init_db() -> Pool<Sqlite> {
    let db_path = "celestial.db";
    let db_url = format!("sqlite://{}", db_path);

    if !Path::new(db_path).exists() {
        fs::File::create(db_path).expect("Failed to create database file");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS celestial_objects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            distance_ly REAL NOT NULL,
            object_type TEXT NOT NULL,
            description TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Check if empty, if so, seed
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM celestial_objects")
        .fetch_one(&pool)
        .await
        .expect("Failed to count rows");

    if count.0 == 0 {
        seed_db(&pool).await;
    }

    pool
}

async fn seed_db(pool: &Pool<Sqlite>) {
    let objects = vec![
        ("Proxima Centauri", 4.24, "Star", "The closest known star to the Sun."),
        ("Alpha Centauri A/B", 4.37, "Star System", "Our nearest bright neighbor system."),
        ("Barnard's Star", 5.96, "Red Dwarf", "A very low-mass red dwarf star."),
        ("Wolf 359", 7.78, "Red Dwarf", "One of the faintest stars near the Sun."),
        ("Lalande 21185", 8.29, "Red Dwarf", "The brightest red dwarf in the northern hemisphere."),
        ("Sirius", 8.6, "Star", "The brightest star in Earth's night sky."),
        ("Epsilon Eridani", 10.5, "Star", "A star resembling a young version of our Sun."),
        ("Procyon", 11.46, "Star", "The eighth brightest star in the night sky."),
        ("Altair", 16.7, "Star", "One of the vertices of the Summer Triangle."),
        ("Fomalhaut", 25.0, "Star", "Known as the 'Lonely One', surrounded by a debris disk."),
        ("Vega", 25.0, "Star", "The standard zero point for the magnitude scale."),
        ("Arcturus", 36.7, "Star", "A red giant and the brightest star in the northern celestial hemisphere."),
        ("TRAPPIST-1", 39.0, "Star System", "Home to seven Earth-sized planets."),
        ("Capella", 42.9, "Star System", "The brightest star in the constellation Auriga."),
        ("Aldebaran", 65.3, "Star", "The fiery eye of Taurus the Bull."),
        ("Regulus", 79.3, "Star", "A quadruple star system in Leo."),
        ("Algol", 90.0, "Star System", "The 'Demon Star', a famous eclipsing binary."),
        ("Dubhe", 123.0, "Star", "One of the pointer stars in the Big Dipper."),
        ("Hyades Cluster", 153.0, "Star Cluster", "The nearest open star cluster to the Solar System."),
        ("Spica", 250.0, "Star", "A blue giant binary star in Virgo."),
        ("Canopus", 310.0, "Star", "The second brightest star in the night sky."),
        ("Polaris", 323.0, "Star", "The current North Star."),
        ("Pleiades (Seven Sisters)", 444.0, "Star Cluster", "A famous open star cluster visible to the naked eye."),
        ("Betelgeuse", 642.5, "Red Supergiant", "A massive star expected to explode as a supernova soon."),
        ("Antares", 550.0, "Red Supergiant", "The 'Heart of the Scorpion'."),
        ("Helix Nebula", 655.0, "Nebula", "One of the closest planetary nebulae to Earth."),
        ("Rigel", 860.0, "Blue Supergiant", "The brightest star in Orion."),
        ("Orion Nebula", 1344.0, "Nebula", "A stellar nursery visible to the naked eye."),
        ("Deneb", 2615.0, "Blue Supergiant", "One of the most luminous stars known."),
        ("Crab Nebula", 6500.0, "Supernova Remnant", "Remnant of the supernova observed in 1054 AD."),
        ("Eagle Nebula (Pillars of Creation)", 7000.0, "Nebula", "Famous for the 'Pillars of Creation' image."),
        ("Eta Carinae", 7500.0, "Star System", "A volatile system that famously erupted in the 1840s."),
        ("Galactic Center (Sagittarius A*)", 26673.0, "Supermassive Black Hole", "The center of our Milky Way galaxy."),
        ("Large Magellanic Cloud", 158200.0, "Galaxy", "A satellite galaxy of the Milky Way."),
        ("Small Magellanic Cloud", 199000.0, "Galaxy", "A dwarf galaxy near the Milky Way."),
        ("Andromeda Galaxy", 2537000.0, "Galaxy", "Our nearest major galactic neighbor, destined to collide with us."),
        ("Triangulum Galaxy", 2723000.0, "Galaxy", "The third-largest member of the Local Group."),
        ("Whirlpool Galaxy", 23000000.0, "Galaxy", "A classic spiral galaxy interacting with a smaller companion."),
        ("Sombrero Galaxy", 31100000.0, "Galaxy", "Famous for its bright nucleus and large central bulge."),
        ("Virgo Cluster", 53800000.0, "Galaxy Cluster", "A massive cluster of galaxies at the center of the Local Supercluster.")
    ];

    for (name, dist, obj_type, desc) in objects {
        sqlx::query("INSERT INTO celestial_objects (name, distance_ly, object_type, description) VALUES (?, ?, ?, ?)")
            .bind(name)
            .bind(dist)
            .bind(obj_type)
            .bind(desc)
            .execute(pool)
            .await
            .expect("Failed to seed database");
    }
    
    log::info!("Database seeded successfully!");
}
