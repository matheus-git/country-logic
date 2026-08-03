% ======== FATOS (gerados pelo Rust) ========

country(brazil).
country(japan).
country(france).

population(brazil, 203000000).
population(japan, 125000000).
population(france, 68000000).

continent(brazil, south_america).
continent(japan, asia).
continent(france, europe).


% ======== REGRAS ========

large_country(Country) :-
    population(Country, Pop),
    Pop > 100000000.

asian_country(Country) :-
    continent(Country, asia).

large_asian_country(Country) :-
    large_country(Country),
    asian_country(Country).
