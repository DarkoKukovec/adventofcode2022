import * as wasm from "adventofcode2022";

window.log = console.log.bind(console);

document.querySelectorAll('.exec').forEach((el) => {
  el.addEventListener('click', () => {
    const id = el.getAttribute('data-id');
    const input = document.querySelector(`#input-${id}`).value;
    const output = document.querySelector(`#output-${id}`);
    el.setAttribute('disabled', 'disabled');
    output.innerHTML = wasm[`exec_${id}`](input) || 'Nothing to execute';
    el.removeAttribute('disabled');
  });
});
