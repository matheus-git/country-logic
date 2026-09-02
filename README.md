# Country Logic

Country Logic is a small command-line application for exploring country data
with Rust and Prolog. It fetches names, regions, populations, ISO codes, and
land borders from the REST Countries API, converts that data into Prolog facts,
and evaluates four simple logical rules with an embedded Scryer Prolog engine.

The project is intentionally compact. Rust handles the command-line interface,
HTTP requests, JSON decoding, and Prolog integration. The rules in `rules.pl`
handle country relationships and population queries.

## Requirements

- Rust with Cargo
- A REST Countries API key

## Setup

Copy the example environment file:

```bash
cp .env.example .env
```

Then replace the placeholder in `.env` with your API key:

```text
RESTCOUNTRIES_API_KEY=your_key
```

The `.env` file is ignored by Git. An existing environment variable takes
precedence over the value in that file.

## Usage

List all countries or regions:

```bash
cargo run -- countries
cargo run -- regions
```

Run country queries:

```bash
cargo run -- large
cargo run -- region Americas
cargo run -- more-populous Brazil
cargo run -- neighbors Brazil
```

Country arguments accept an English name or a three-letter ISO code. Lists are
sorted alphabetically and contain no duplicates.

## Prolog rules

- `large_country(Country)` finds countries with more than 100 million people.
- `country_in_region(Country, Region)` finds countries in a region.
- `more_populous(CountryA, CountryB)` finds countries with a larger population.
- `borders(Country, Neighbor)` finds countries that share a land border.

## Project structure

- `src/main.rs` defines the CLI and connects Rust to Prolog.
- `src/query_countries.rs` fetches and decodes the required API data.
- `rules.pl` contains the four Prolog rules.
- `.env.example` documents the required environment variable.

## Validation

Run the automated checks with:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```
