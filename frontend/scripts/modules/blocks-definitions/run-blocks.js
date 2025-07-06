export const runBlocks = {}


const easyRun = {
    type: "easy_run", 
    colour:  "#A5D49D",
    message0: "%1 Iniciar",
    args0: [
        {
            type: "field_image",
            src: "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'%3e%3cpath fill='%233CB827' d='M12 4c-4.41 0-8 3.59-8 8s3.59 8 8 8 8-3.59 8-8-3.59-8-8-8M9.5 16.5v-9l7 4.5z' opacity='.3'/%3e%3cpath fill='%233CB827' d='M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2m0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8'/%3e%3cpath fill='%233CB827' d='m9.5 16.5 7-4.5-7-4.5z'/%3e%3c/svg%3e",
            width: 30,
            height: 30,
            alt: "▶️",
            name: "PLAY_BUTTON"
        }
    ],
    nextStatement: null,
    tooltip: "Inicia o código",
}
runBlocks["easy_run"] = easyRun;
