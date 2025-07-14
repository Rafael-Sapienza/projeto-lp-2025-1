export const creatorDivHTML = `
    <div class="form-content">
        <!-- your existing form here -->
        <div class="form-line">
            <label>Nome do Bloco:</label>
            <input id="function-name" type="text" />
        </div>
        <div class="form-line">
            <label>Adicionar Caixas:</label>
            <select id="function-parameters">
                <option value="NONE">Opcional</option>
                <option value="Number">Número</option>
                <option value="String">Texto</option>
                <option value="Boolean">Verdadeiro/Falso</option>
            </select>
        </div>
        <div id="parameters-container"></div>
        <div class="form-line">
            <label>Devolver:</label>
            <select id="function-return">
                <option value="NONE">Opcional</option>
                <option value="Number">Número</option>
                <option value="String">Texto</option>
                <option value="Boolean">Verdadeiro/Falso</option>
            </select>
        </div>
        <div class="form-line">
            <button id="cancel-creation">Cancelar</button>
            <button id="confirm-creation">Criar</button>
        </div>
    </div>
`;
