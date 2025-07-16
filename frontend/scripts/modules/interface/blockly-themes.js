export const easyTheme = Blockly.Theme.defineTheme( 
    'easy_theme', 
    {
        'base': Blockly.Themes.Classic,
        'fontStyle': {
            'family': 'Sour Gummy',
            'size': 18,
        },
        componentStyles: {
            workspaceBackgroundColour: 'rgba(233, 199, 176, 0.85)',
            toolboxBackgroundColour: '#FCE4D6',
            toolboxForegroundColour: '#3E2723',
            flyoutBackgroundColour: '#FFF3E0',
            flyoutForegroundColour: '#4E342E',
            flyoutOpacity: 0.95,
            scrollbarColour: '#9e665c9c',
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
      workspaceBackgroundColour: '#102C3A',           
      toolboxBackgroundColour: '#1A3D4F',             
      toolboxForegroundColour: '#D0F4F7',             
      flyoutBackgroundColour: '#153544',              
      flyoutForegroundColour: '#88C0D0',              
      flyoutOpacity: 0.95,
      scrollbarColour: '#88C0D0',                     
      insertionMarkerColour: '#FF8A65',               
      insertionMarkerOpacity: 0.5,
      markerColour: '#FFD54F',                        
      cursorColour: '#80DEEA'                         
    },

        startHats: true,
    }
);
