export const easyTheme = Blockly.Theme.defineTheme( 
    'easy_theme', 
    {
        'base': Blockly.Themes.Classic,
        'fontStyle': {
            'family': 'Sour Gummy',
            'size': 18,
        },
        componentStyles: {
            workspaceBackgroundColour: '#FFF9F0',
            toolboxBackgroundColour: '#FCE4D6',
            toolboxForegroundColour: '#3E2723',
            flyoutBackgroundColour: '#FFF3E0',
            flyoutForegroundColour: '#4E342E',
            flyoutOpacity: 0.95,
            scrollbarColour: '#D7CCC8',
            insertionMarkerColour: '#FF7043',
            insertionMarkerOpacity: 0.5,
            markerColour: '#F50057',
            cursorColour: ''
            },
        startHats: true,
    }
);
