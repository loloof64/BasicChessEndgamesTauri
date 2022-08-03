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
                'options-page-tooltip': 'Aller à la page des paramètres',
            },
            'dialogs': {
                'new-game-confirmation': 'Souhaitez-vous arrêter la partie en cours et en démarrer une nouvelle ?',
                'stop-game-confirmation': 'Souhaitez-vous arrêter la partie en cours ?',
            }
        },
        'options' : {
            'fields': {
                'engine-path-label': 'Chemin moteur d\'échecs UCI',
                'engine-path-button': 'Choisir',
            },
            'dialogs': {
                'validate-confirmation': 'Voulez-vous sauvegarder les paramètres et quitter ?',
                'cancel-confirmation': 'Voulez-vous annuler les paramètres et quitter ?',
                'select-engine-title': 'Choisir le moteur d\'échecs UCI',
                'not-uci-engine': 'Le programme choisi n\'est pas un moteur d\'échecs UCI, ou dispose de droits d\'accès trop restrictifs.',
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