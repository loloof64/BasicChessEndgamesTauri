export default {
    'pages': {
        'game': {
            'status': {
                'checkmate': `{side} has won by checkmate.`,
                'stalemate': 'Drawy by stalemate.',
                'three-fold-repetition': 'Draw by 3-fold repetition.',
                'missing-material': 'Draw by missing material.',
                'fifty-moves': 'Draw by the 50 moves rule.',
                'new-game': 'New game started.',
                'game-stopped': 'Game stopped.',
            },
            'buttons': {
                'new-game-tooltip': 'New game',
                'stop-game-tooltip': 'Stop game',
                'reverse-board-tooltip': 'Reverse board',
                'options-page-tooltip': 'Go to settings page',
            },
            'dialogs': {
                'new-game-confirmation': 'Do you want to stop current game and start a new one ?',
                'stop-game-confirmation': 'Do you want to stop current game ?',
            }
        },
        'options' : {
            'fields': {
                'engine-path-label': 'Chess UCI engine path',
                'engine-path-button': 'Select',
            },
            'dialogs': {
                'validate-confirmation': 'Do you want to save settings and exit page ?',
                'cancel-confirmation': 'Do you want to cancel settings and exit page ?',
                'select-engine-title': 'Select the chess UCI engine.',
                'not-uci-engine': 'The selected program is not a chess UCI engine, or has too restricted access rights.',
            }
        }
    },
    'dialogs': {
        'ok': "OK",
        'cancel': "Cancel",
    },
    'chess': {
        'white': 'white',
        'black': 'black',
    }
}