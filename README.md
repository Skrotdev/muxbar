# Muxbar

Tmux status line configured in Rust.

## Features

- Fully configured in Rust
  - Type-save configuration
  - Can be programmed (e.g. dynamically rendered modules)
- Supports formatting
- Cached modules
    - Each module specifies when it needs to recompute and also how to update it self.
    - Once a module needs to recompute that specific module is updated
    - All other modules are cached
- Kubernetes context + namespace status
  - Displays current `kubectl` context and namespace inline
- Dynamic weather module
  - Fetches live temperature and weather conditions using Open-Meteo API
  - Nerd Font icons adjust based on time (day/night) and conditions (rain, fog, etc.)

## Installation

1. Clone this repository

   ```bash
   git clone git@github.com:Dlurak/muxbar.git
   ```

2. Install Muxbar

   ```bash
   cargo install --path .
   ```

3. Apply Muxbar in your `.tmux.conf`

   ```text
   set -g status-right '#(muxbar)'
   ```

## Configuration

The configuration is written in Rust and located in `./src/config.rs`

## Examples

<img width="904" height="24" alt="image" src="https://github.com/user-attachments/assets/0f57d9f7-fcfe-4d81-80af-07b20a9e4ea7" />

