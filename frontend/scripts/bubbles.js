document.querySelectorAll('.mode-button').forEach(button => {
  button.addEventListener('mouseenter', () => {
    // cria 5 bolhas por hover
    for (let i = 0; i < 5; i++) {
      const bubble = document.createElement('span');
      bubble.classList.add('btn-bubble');

      // tamanho entre 6 e 14px
      const size = Math.random() * 8 + 6;
      bubble.style.width = size + 'px';
      bubble.style.height = size + 'px';

      // posição horizontal aleatória dentro do botão
      const x = Math.random() * button.clientWidth;
      bubble.style.left = x + 'px';

      // posição vertical na base do botão
      bubble.style.bottom = '10px';

      // adiciona ao botão
      button.appendChild(bubble);

      // remove a bolha depois da animação (1s)
      setTimeout(() => {
        bubble.remove();
      }, 1000);
    }
  });
});

const bubblesContainer = document.querySelector('.bubbles');

for(let i = 0; i < 30; i++) {
  const bubble = document.createElement('span');
  bubble.style.left = Math.random() * 100 + '%';
  const size = Math.random() * 25 + 10; // 10 a 35 px
  bubble.style.width = size + 'px';
  bubble.style.height = size + 'px';
  bubble.style.setProperty('--i', Math.random() * 20 + 5); // duração variada
  bubblesContainer.appendChild(bubble);
}
