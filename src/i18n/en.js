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
            },
            'dialogs': {
                'new-game-confirmation': 'Do you want to stop current game and start a new one ?',
                'stop-game-confirmation': 'Do you want to stop current game ?',
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