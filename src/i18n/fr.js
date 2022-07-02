export default {
    'pages': {
        'game': {
            'status': {
                'checkmate': `{side} ont gagné par échec et mat.`,
                'stalemate': 'Nulle par pat.',
                'three-fold-repetition': 'Nulle par triple répétition.',
                'missing-material': 'Nulle par manque de matériel.',
                'fifty-moves': 'Nulle par la règle des 50 coups.',
                'new-game': 'Nouvelle partie démarée.',
                'game-stopped': 'Partie arrêtée.',
            },
            'buttons': {
                'new-game-tooltip': 'Nouvelle partie',
                'stop-game-tooltip': 'Arrêter la partie',
                'reverse-board-tooltip': 'Renverser l\'échiquier',
            },
            'dialogs': {
                'new-game-confirmation': 'Souhaitez-vous arrêter la partie en cours et en démarrer une nouvelle ?',
                'stop-game-confirmation': 'Souhaitez-vous arrêter la partie en cours ?',
            }
        }
    },
    'dialogs': {
        'ok': "D'accord",
        'cancel': "Annuler",
    },
    'chess': {
        'white': 'les Blancs',
        'black': 'les Noirs',
    }
}