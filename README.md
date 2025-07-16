# 🎯 Favorites Solana Anchor Program

This is a basic Solana smart contract built with [Anchor](https://project-serum.github.io/anchor/) that allows users to store their favorite number, color, and a list of hobbies on-chain.

> Ideal for beginners learning how to build Solana programs using Anchor!

---

## 📦 Features

- 📝 Users can store:
  - A `favorite number` (u64)
  - A `favorite color` (string)
  - A list of up to 5 `hobbies` (strings)
- 🧠 Uses **PDAs (Program Derived Addresses)** to store user-specific data
- 💰 Data is stored on-chain with proper memory allocation using `INIT_SPACE`

---

## 🧠 Program Overview

The program has:

- A `Favorites` account struct to hold the user data.
- A `set_favorites` instruction to save the data on-chain.
- Anchor's declarative macros for account initialization and memory management.

---

## 📁 File Structure
favorites-solana-anchor/
│
├── programs/
│ └── favorites/
│ └── src/lib.rs # Main smart contract code
│
├── Anchor.toml # Anchor project config
└── Cargo.toml # Rust dependencies

---

## 🔨 How to Deploy (Locally or on Devnet)

```bash
# Install dependencies
anchor build

# Start a local Solana test validator
solana-test-validator

# Deploy the program
anchor deploy


✍️ Usage (Example Client Code)
You can write a TypeScript or CLI client to interact with the deployed program using Anchor or Solana Web3.js.



🧪 Sample Output (Logs)
Greeting from <Program ID>
User <wallet address>'s favorite number is 42, favorite color is "blue", and their hobbies are ["coding", "reading", "music"]


📜 Account Schema

pub struct Favorites {
    pub number: u64,
    pub colour: String,
    pub hobbies: Vec<String>,
}
colour: Max 50 characters

hobbies: Up to 5 items, each max 50 characters

🛠 Built With
🦀 Rust

⚓ Anchor Framework

🔗 Solana Blockchain

🤝 Contributing
Pull requests are welcome! For major changes, please open an issue first.

📄 License
This project is open-source and available under the MIT License.

🙋‍♂️ Author
Nakul Sali


---

Let me know if you want a logo, usage instructions with a CLI, or a GIF demo added to this README!
