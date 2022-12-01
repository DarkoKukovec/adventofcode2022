import * as wasm from "adventofcode2022";

window.log = console.log.bind(console);

document.querySelectorAll('.input').forEach((el) => {
  const lines = el.value.split('\n').length;
  el.setAttribute('rows', lines);

  el.addEventListener('input', () => {
    const lines = el.value.split('\n').length;
    el.setAttribute('rows', lines);
  });
});

document.querySelectorAll('h3').forEach((el) => {
  const id = el.parentElement.getAttribute('data-id');
  el.innerHTML = `Day ${id}`;
  el.addEventListener('click', () => {
    const opened = document.querySelector('.opened')
    if (opened) opened.classList.remove('opened');
    el.parentElement.classList.add('opened');
  });
});

document.querySelector('h3').parentElement.classList.add('opened');

document.querySelectorAll('.exec').forEach((el) => {
  el.addEventListener('click', () => {
    const dataIndex = document.querySelector('[type=radio]:checked').value === 'test' ? 0 : 1;
    const target = el.parentElement.parentElement;
    const id = target.getAttribute('data-id');
    const input = target.querySelector('.input').value || require(`./inputs/${id}.json`)[dataIndex];
    const output = target.querySelector('.output');
    el.setAttribute('disabled', 'disabled');
    output.innerHTML = wasm[`exec_${id}`](input) || 'Nothing to execute';
    el.removeAttribute('disabled');
  });
});
