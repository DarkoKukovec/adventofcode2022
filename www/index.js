import * as wasm from 'adventofcode2022';

window.log = console.log.bind(console);

const maxDay = 5;

document.querySelector('.app').innerHTML = Array.from({ length: maxDay + 1 })
	.map(
		(_, day) => `<section data-id="${day || 'test'}" class="${day === maxDay ? 'opened' : ''}">
  <h3>Day ${day || 'test'}</h3>
  <div>
    <div class="wrap">
      <div>
        Input: <textarea class="input"></textarea>
      </div>
      <div>
        Output: <pre class="output"></pre>
      </div>
    </div>
    <button class="exec" data-input="input">Execute input</button>
    <button class="exec" data-input="test">Execute test</button>
    <button class="exec" data-input="task">Execute task</button>
  </div>
  </section>`,
	)
	.reverse()
	.join('');

document.querySelectorAll('.input').forEach((el) => {
	const lines = el.value.split('\n').length;
	el.setAttribute('rows', lines);

	el.addEventListener('input', () => {
		const lines = el.value.split('\n').length;
		el.setAttribute('rows', lines);
	});
});

document.querySelectorAll('h3').forEach((el) => {
	el.addEventListener('click', () => {
		const opened = document.querySelector('.opened');
		if (opened) opened.classList.remove('opened');
		el.parentElement.classList.add('opened');
	});
});

document.querySelectorAll('.exec').forEach((el) => {
	el.addEventListener('click', () => {
		const target = el.parentElement.parentElement;
		const dataType = el.getAttribute('data-input');
		const id = target.getAttribute('data-id');
		const input =
			dataType === 'input'
				? el.parentElement.querySelector('.input').value
				: dataType === 'test'
				? require(`./inputs/${id}.json`)[0]
				: require(`./inputs/${id}.json`)[1];
		const output = target.querySelector('.output');
		el.setAttribute('disabled', 'disabled');
		try {
			const start = performance.now();
			output.innerHTML = wasm[`exec_${id}`](input) || 'Nothing to execute';
			const end = performance.now();
			output.innerHTML += `<br />Executed in ${(end - start).toFixed(1)}ms`;
		} catch (e) {
			output.innerHTML = e;
		}
		el.removeAttribute('disabled');
	});
});
