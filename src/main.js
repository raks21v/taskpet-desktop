const petButton = document.querySelector('.pet');
const speechBubble = document.querySelector('.speech-bubble');

petButton.addEventListener('click', (event) => {
  event.stopPropagation();
  speechBubble.classList.toggle('is-open');
});

window.addEventListener('click', () => {
  speechBubble.classList.remove('is-open');
});
