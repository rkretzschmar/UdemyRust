# TODO

- Die number_split in primes ist nach meiner Meinung nicht ganz korrekt.
    number_split wird in der for Schleife mit den Werten 0..10 aufgerufen.
    Für 0 und 1 gibt sie 5000 zurück, dh. die Primzahlen ab 5000 werden 2 mal berechnet.

    Besser wäre:
    fn number_split(id: u64) -> (u64, u64) {
    let offset_start = 500_000_000 + id * 100;
    let offset_end = offset_start + 100;
    (offset_start, offset_end)
    }
- macro_rules
- Unit Tests
- Handbuch Video updaten wegen repo link
