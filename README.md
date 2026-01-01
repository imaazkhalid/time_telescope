# Time Telescope 🔭

**Time Telescope** is a Rust-based web application that allows you to calculate the cosmic distance required to "look back in time" at Earth. Based on the principle that light travels at a finite speed, it determines how far away in space you would need to be to observe specific historical events on Earth as they happened.

It's not just a calculator; it's a **cosmic travel guide**. The app matches your target date with real astronomical landmarks (stars, nebulae, galaxies) and tells you exactly where you'd need to be standing in the universe to watch history unfold.

![Time Telescope Screenshot](https://github.com/imaazkhalid/time_telescope/blob/master/static/image.png?raw=true)

## ✨ Features

*   **Deep Time Calculation:** Calculate distances for *any* date in history, from yesterday to ancient Egypt (BCE support included).
*   **Cosmic Landmarks:** Automatically finds the nearest real star, galaxy, or nebula to your target distance. "To see the year 1998, go to Vega."
*   **Voyager 1 Context:** Calculates how long it would take to travel there at current human maximum speeds (Voyager 1 speed ~17 km/s).
*   **Immersive UI:** A beautiful, space-themed interface with a procedurally generated animated starfield.
*   **Historical Milestones:** Quick-access buttons for major events like the Moon Landing, End of WWII, and the First Writing.

## 🚀 Tech Stack

*   **Backend:** Rust 🦀
    *   `actix-web`: High-performance web framework.
    *   `sqlx`: Type-safe, async SQLite database interaction.
    *   `chrono`: Robust date and time handling.
*   **Frontend:** HTML5, CSS3, Vanilla JavaScript.
*   **Database:** SQLite (embedded, zero-configuration).

## 🛠️ Installation & Setup

### Prerequisites
*   **Rust & Cargo:** You need to have Rust installed. If you don't, install it via [rustup.rs](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **Git:** To clone the repository.

### Quick Start

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/imaazkhalid/time_telescope.git
    cd time_telescope
    ```

2.  **Run the application:**
    Cargo handles everything, including building dependencies and setting up the database on the first run.
    ```bash
    cargo run
    ```

3.  **Access the App:**
    Open your web browser and navigate to:
    👉 **http://127.0.0.1:8080**

## 📖 How to Use

1.  **Enter a Date:**
    *   Use the "Year" field for any year (e.g., `2023`, `1990`).
    *   For **BCE** (Before Common Era) dates, use negative numbers (e.g., `-3200` for 3200 BCE).
    *   Select the Month, Day, and Time.
2.  **Click "Focus Telescope":**
    *   The app will calculate the exact distance in Light Years, Kilometers, and Miles.
3.  **Explore the Results:**
    *   See which **Cosmic Landmark** (star/galaxy) is your viewing post.
    *   See how many thousands (or millions!) of years it would take to travel there today.

## 🤝 Contributing

Contributions are welcome! If you have a favorite star or galaxy you'd like to add to the database, feel free to open a Pull Request.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request
