export default {
    'pages': {
        'game': {
            'status': {
                'checkmate': `{side} ha ganado por jaque mate.`,
                'stalemate': 'Empate por ahogado',
                'three-fold-repetition': 'Empate por tres repeticiones.',
                'missing-material': 'Empate por falta de material.',
                'fifty-moves': 'Empate por la regla de los 50 movimientos.',
                'new-game': 'Nueva partida empezada.',
                'game-stopped': 'Partida salida.',
            },
            'buttons': {
                'new-game-tooltip': 'Nueva partida',
                'stop-game-tooltip': 'Salir partida',
                'reverse-board-tooltip': 'Volcar el tablero de ajedrez',
                'options-page-tooltip': 'Ir a la página de configuración',
            },
            'dialogs': {
                'new-game-confirmation': '¿ Quiere salir de esta partida y empezar una nueva ?',
                'stop-game-confirmation': '¿ Quiere salir de esta partida ?',
            }
        },
        'options' : {
            'fields': {
                'engine-path-label': 'Ruta del motor de ajedrez UCI',
                'engine-path-button': 'Seleccionar',
            },
            'dialogs': {
                'validate-confirmation': '¿ Desea guardar la configuración y salir de la página ?',
                'cancel-confirmation': '¿ Desea anular la configuración y salir de la página ?',
                'select-engine-title': 'Seleccione el motor de ajedrez UCI.',
                'not-uci-engine': 'La programa seleccionada no es un motor UCI de ajedrez, o tiene derechos de acceso demasiado restringidos.',
            }
        }
    },
    'dialogs': {
        'ok': "De accuerdo",
        'cancel': "Anular",
    },
    'chess': {
        'white': 'los Blancos',
        'black': 'los Negros',
    }
}