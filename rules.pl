:- dynamic(population/2).
:- dynamic(region/2).
:- dynamic(alpha3/2).
:- dynamic(border_code/2).

large_country(Country) :-
    population(Country, Population),
    Population > 100000000.

country_in_region(Country, Region) :-
    region(Country, Region).

more_populous(CountryA, CountryB) :-
    population(CountryA, PopulationA),
    population(CountryB, PopulationB),
    PopulationA > PopulationB.

borders(Country, Neighbor) :-
    border_code(Country, NeighborCode),
    alpha3(Neighbor, NeighborCode).
