"""Generate a 10000-row x 20-column CSV based on test_stocks.csv schema."""

import csv
import random
import string

random.seed(42)

SECTORS = ["Technology", "Financial", "Healthcare", "Automotive", "Retail",
           "Energy", "Industrial", "Telecom", "Consumer", "Materials"]

PREFIXES = [
    "Alpha", "Beta", "Gamma", "Delta", "Sigma", "Omega", "Nova", "Apex",
    "Core", "Prime", "Neo", "Zen", "Arc", "Bolt", "Cyber", "Digi",
    "Eco", "Fusion", "Grid", "Hyper", "Ion", "Jet", "Kilo", "Luna",
    "Macro", "Nano", "Orbit", "Pulse", "Quantum", "Rapid", "Solar",
    "Turbo", "Ultra", "Vertex", "Wave", "Xeno", "Yotta", "Zeta",
    "Blue", "Red", "Green", "Gold", "Silver", "Iron", "Star", "Cloud",
    "Peak", "Bay", "Crest", "Summit",
]

SUFFIXES = [
    "Tech", "Corp", "Inc", "Systems", "Solutions", "Group", "Holdings",
    "Industries", "Labs", "Networks", "Dynamics", "Logic", "Ware",
    "Works", "Soft", "Net", "Data", "AI", "IO", "Cloud",
]


def gen_symbol(row_id: int) -> str:
    """Generate a ticker-like symbol."""
    letters = "".join(random.choices(string.ascii_uppercase, k=random.choice([3, 4, 5])))
    return letters


def gen_name(row_id: int) -> str:
    prefix = random.choice(PREFIXES)
    suffix = random.choice(SUFFIXES)
    return f"{prefix}{suffix} Corp."


def main():
    num_rows = 10_000
    columns = [
        "symbol", "name", "price", "change_pct", "volume", "market_cap",
        "pe_ratio", "sector",
        # --- 12 additional columns ---
        "dividend_yield",      # 9
        "fifty_two_week_high", # 10
        "fifty_two_week_low",  # 11
        "beta",                # 12
        "eps",                 # 13
        "revenue",             # 14
        "debt_to_equity",      # 15
        "current_ratio",       # 16
        "roe",                 # 17
        "analyst_rating",      # 18
        "avg_volume",          # 19
        "exchange",            # 20
    ]

    exchanges = ["NYSE", "NASDAQ", "AMEX", "LSE", "TSE", "HKEX", "SSE"]

    out_path = "/home/gewei/workspace/csv-viewer-egui/test_stocks_10k.csv"

    with open(out_path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(columns)

        used_symbols: set[str] = set()
        for i in range(num_rows):
            # unique symbol
            while True:
                sym = gen_symbol(i)
                if sym not in used_symbols:
                    used_symbols.add(sym)
                    break

            name = gen_name(i)
            price = round(random.uniform(5.0, 1500.0), 2)
            change_pct = round(random.uniform(-8.0, 8.0), 2)
            volume = random.randint(100_000, 100_000_000)
            market_cap = random.randint(1_000_000_000, 3_000_000_000_000)
            pe_ratio = round(random.uniform(5.0, 120.0), 1)
            sector = random.choice(SECTORS)
            dividend_yield = round(random.uniform(0.0, 6.0), 2)
            high52 = round(price * random.uniform(1.05, 1.60), 2)
            low52 = round(price * random.uniform(0.40, 0.90), 2)
            beta = round(random.uniform(0.3, 2.5), 2)
            eps = round(random.uniform(-2.0, 30.0), 2)
            revenue = random.randint(500_000_000, 500_000_000_000)
            debt_to_equity = round(random.uniform(0.1, 3.0), 2)
            current_ratio = round(random.uniform(0.5, 4.0), 2)
            roe = round(random.uniform(-10.0, 50.0), 1)
            analyst_rating = round(random.uniform(1.0, 5.0), 1)
            avg_volume = random.randint(50_000, 50_000_000)
            exchange = random.choice(exchanges)

            writer.writerow([
                sym, name, price, change_pct, volume, market_cap,
                pe_ratio, sector, dividend_yield, high52, low52,
                beta, eps, revenue, debt_to_equity, current_ratio,
                roe, analyst_rating, avg_volume, exchange,
            ])

    print(f"Done → {out_path}  ({num_rows} rows × {len(columns)} cols)")


if __name__ == "__main__":
    main()
