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


export const abyssTheme = Blockly.Theme.defineTheme(
  'abyssTheme',
  {
    base: Blockly.Themes.Classic,
    fontStyle: {
      family: 'UoqMunThenKhung',
      size: 15,
      weight: 'bold',
      
    },
    componentStyles: {
      workspaceBackgroundColour: '#102C3A',           // azul-petróleo escuro (base do fundo)
      toolboxBackgroundColour: '#1A3D4F',             // tom mais intenso para contraste
      toolboxForegroundColour: '#D0F4F7',             // azul claro visível
      flyoutBackgroundColour: '#153544',              // tom semelhante à sombra do fundo
      flyoutForegroundColour: '#88C0D0',              // destaque suave em azul
      flyoutOpacity: 0.95,
      scrollbarColour: '#88C0D0',                     // azul claro para detalhes visíveis
      insertionMarkerColour: '#FF8A65',               // coral suave para visibilidade
      insertionMarkerOpacity: 0.5,
      markerColour: '#FFD54F',                        // dourado claro para marcar
      cursorColour: '#80DEEA'                         // azul neon sutil
    },

        startHats: true,
    }
);
