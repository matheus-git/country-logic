:- use_module(library(lists)).

resultado('v', 1).
resultado('e', 0.5).
resultado('d', 0).

rodada(Resultados, Pontos, Partidas) :-
    length(Resultados, Partidas),
    rodada_(Resultados, Pontos).

rodada_([], 0).

rodada_([R|Rs], Soma) :-
    resultado(R, Valor),
    rodada_(Rs, Soma1),
    Soma is Valor + Soma1.
